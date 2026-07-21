use serde::{Deserialize, Serialize};

/// Service tier classification, where `T1` is the most demanding and `T4` is baseline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    /// Returns a stable, human-readable label for the tier.
    pub fn label(&self) -> &'static str {
        match self {
            Tier::T1 => "T1",
            Tier::T2 => "T2",
            Tier::T3 => "T3",
            Tier::T4 => "T4",
        }
    }
}

/// Service-level agreement targets for a tier.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sla {
    pub access_time: f64,
    pub capacity_seats: f64,
    pub program_breadth: f64,
    pub outcomes: f64,
}

/// Returns the provisional, hardcoded SLA table for a tier.
///
/// `T1` is the most demanding tier and `T4` is the baseline.
pub fn provisional_sla(tier: Tier) -> Sla {
    match tier {
        Tier::T1 => Sla {
            access_time: 15.0,
            capacity_seats: 1000.0,
            program_breadth: 40.0,
            outcomes: 95.0,
        },
        Tier::T2 => Sla {
            access_time: 25.0,
            capacity_seats: 600.0,
            program_breadth: 30.0,
            outcomes: 85.0,
        },
        Tier::T3 => Sla {
            access_time: 35.0,
            capacity_seats: 300.0,
            program_breadth: 20.0,
            outcomes: 75.0,
        },
        Tier::T4 => Sla {
            access_time: 45.0,
            capacity_seats: 100.0,
            program_breadth: 10.0,
            outcomes: 65.0,
        },
    }
}

/// Classifies a corpus entry into a [`Tier`], defaulting to [`Tier::T4`].
pub fn classify(entry: &slate_corpus::CorpusEntry) -> Tier {
    match entry.tier.as_deref() {
        Some("T1") => Tier::T1,
        Some("T2") => Tier::T2,
        Some("T3") => Tier::T3,
        Some("T4") => Tier::T4,
        _ => Tier::T4,
    }
}

/// Conformance assessment for dimension 13 (tier SLA capacity).
#[derive(Clone, Copy, Debug)]
pub struct Dim13 {
    pub score: slate_score::Score,
    pub basis: slate_corpus::DemandBasis,
    pub redundancy: bool,
}

fn observed_seats(entry: &slate_corpus::CorpusEntry) -> f64 {
    entry
        .quantities
        .iter()
        .find(|q| q.unit.to_lowercase().contains("seats"))
        .map(|q| q.value)
        .unwrap_or(0.0)
}

/// Computes the dimension-13 conformance of an entry against the network topology.
pub fn conformance(entry: &slate_corpus::CorpusEntry, network: &slate_network::Network) -> Dim13 {
    let required = provisional_sla(classify(entry));
    let observed = observed_seats(entry);
    let redundancy = matches!(network.degree(&entry.id), Some(d) if d >= 2);

    let mut result = (observed / required.capacity_seats).min(1.0) * 10.0;
    if !redundancy {
        result -= 2.0;
    }

    Dim13 {
        score: slate_score::Score::clamped(result),
        basis: slate_corpus::DemandBasis::Surge,
        redundancy,
    }
}

/// Reported gap between an entry's observed seats and its tier's required capacity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TierSlaGap {
    pub entry_id: String,
    pub tier: Tier,
    pub required_seats: f64,
    pub observed_seats: f64,
    pub label: slate_corpus::EvidenceLabel,
}

/// Returns a provisional tier SLA gap when observed seats fall short of the requirement.
pub fn tier_sla_gap(entry: &slate_corpus::CorpusEntry) -> Option<TierSlaGap> {
    let tier = classify(entry);
    let required = provisional_sla(tier);
    let observed = observed_seats(entry);

    if observed < required.capacity_seats {
        Some(TierSlaGap {
            entry_id: entry.id.clone(),
            tier,
            required_seats: required.capacity_seats,
            observed_seats: observed,
            label: slate_corpus::EvidenceLabel::Provisional,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_network() -> slate_network::Network {
        let mut net = slate_network::Network::new();
        net.add_school(slate_network::School {
            id: String::from("A"),
            name: String::from("A"),
            role: slate_network::SchoolRole::Secondary,
        })
        .unwrap();
        net.add_school(slate_network::School {
            id: String::from("B"),
            name: String::from("B"),
            role: slate_network::SchoolRole::Elementary,
        })
        .unwrap();
        net.add_school(slate_network::School {
            id: String::from("C"),
            name: String::from("C"),
            role: slate_network::SchoolRole::Elementary,
        })
        .unwrap();
        net.add_pathway(
            "A",
            "B",
            slate_network::Pathway {
                id: String::from("p1"),
                capacity_seats: 10.0,
                basis: slate_network::DemandBasis::Surge,
            },
        )
        .unwrap();
        net.add_pathway(
            "B",
            "C",
            slate_network::Pathway {
                id: String::from("p2"),
                capacity_seats: 5.0,
                basis: slate_network::DemandBasis::Surge,
            },
        )
        .unwrap();
        net.add_pathway(
            "A",
            "C",
            slate_network::Pathway {
                id: String::from("p3"),
                capacity_seats: 7.0,
                basis: slate_network::DemandBasis::Surge,
            },
        )
        .unwrap();
        net
    }

    fn seats_entry(id: &str, tier: &str, seats: f64) -> slate_corpus::CorpusEntry {
        slate_corpus::CorpusEntry {
            id: String::from(id),
            tier: Some(String::from(tier)),
            quantities: vec![slate_corpus::Quantity {
                value: seats,
                unit: String::from("seats"),
                label: slate_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        }
    }

    #[test]
    fn classify_maps_all_tiers_and_defaults_to_t4() {
        let t1 = slate_corpus::CorpusEntry {
            tier: Some(String::from("T1")),
            ..Default::default()
        };
        let t2 = slate_corpus::CorpusEntry {
            tier: Some(String::from("T2")),
            ..Default::default()
        };
        let t3 = slate_corpus::CorpusEntry {
            tier: Some(String::from("T3")),
            ..Default::default()
        };
        let t4 = slate_corpus::CorpusEntry {
            tier: Some(String::from("T4")),
            ..Default::default()
        };
        let none = slate_corpus::CorpusEntry {
            tier: None,
            ..Default::default()
        };

        assert_eq!(classify(&t1), Tier::T1);
        assert_eq!(classify(&t2), Tier::T2);
        assert_eq!(classify(&t3), Tier::T3);
        assert_eq!(classify(&t4), Tier::T4);
        assert_eq!(classify(&none), Tier::T4);
    }

    #[test]
    fn conforming_entry_has_no_gap_and_high_score() {
        let net = make_network();
        // T4 requires 100 seats; 500 observed seats meets/exceeds it on a redundant node.
        let entry = seats_entry("A", "T4", 500.0);

        assert!(tier_sla_gap(&entry).is_none());

        let dim = conformance(&entry, &net);
        assert!(dim.score.value() >= 9.0);
    }

    #[test]
    fn shortfall_yields_provisional_gap() {
        let entry = slate_corpus::CorpusEntry {
            id: String::from("A"),
            tier: Some(String::from("T1")),
            quantities: vec![slate_corpus::Quantity {
                value: 500.0,
                unit: String::from("seats"),
                label: slate_corpus::EvidenceLabel::Cited,
                source_id: Some(String::from("s1")),
            }],
            ..Default::default()
        };

        let gap = tier_sla_gap(&entry);
        assert!(gap.is_some());
        let gap = gap.unwrap();
        assert!(matches!(
            gap.label,
            slate_corpus::EvidenceLabel::Provisional
        ));
    }

    #[test]
    fn diverse_path_scores_higher_than_constrained() {
        let net = make_network();
        // "A" sits on a diverse path (degree >= 2); "Z" is absent (degree below 2).
        let diverse = seats_entry("A", "T1", 500.0);
        let constrained = seats_entry("Z", "T1", 500.0);

        let diverse_score = conformance(&diverse, &net);
        let constrained_score = conformance(&constrained, &net);

        assert!(diverse_score.redundancy);
        assert!(!constrained_score.redundancy);
        assert!(diverse_score.score.value() > constrained_score.score.value());
    }
}
