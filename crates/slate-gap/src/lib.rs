//! Gap analysis for the SLATE project.
//!
//! This crate identifies "gap regions" within a corpus: dimensions on which the
//! selected entries score poorly on average. It also surfaces tier SLA gaps that
//! pertain to the selected entries.
//!
//! # Cross-scale analysis
//!
//! By default [`find_gaps`] only considers entries whose scale matches the
//! requested [`slate_corpus::Scale`]. Cross-scale analysis, which considers
//! every entry in the corpus regardless of its scale, requires passing
//! `cross_scale = true`.

use serde::{Deserialize, Serialize};
use slate_score::DimensionScorer;
use thiserror::Error;

/// Errors that may arise during gap analysis.
///
/// Reserved for future fallible operations; gap analysis is currently
/// infallible but exposes a typed error for forward compatibility.
#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum GapError {
    /// An entry referenced a dimension that the rubric does not define.
    #[error("unknown dimension: {0}")]
    UnknownDimension(String),
}

/// A region of weakness: a dimension/scale pairing whose selected entries
/// score below the adequacy threshold on average.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapRegion {
    /// The dimension on which the selected entries are weak.
    pub dimension: slate_score::Dimension,
    /// The scale this gap region pertains to.
    pub scale: slate_corpus::Scale,
    /// The mean score across the selected entries for this dimension.
    pub mean_score: f64,
    /// The ids of the entries that contributed to this region.
    pub member_ids: Vec<String>,
    /// Evidence label describing the strength of this finding.
    pub label: slate_corpus::EvidenceLabel,
}

/// A dimension whose under-served tail falls below the adequacy threshold even
/// when the corpus mean does not.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TailGapRegion {
    /// The dimension on which the selected entries have an under-served tail.
    pub dimension: slate_score::Dimension,
    /// The scale this tail gap region pertains to.
    pub scale: slate_corpus::Scale,
    /// Mean score of the bottom-quartile entries for this dimension.
    pub tail_mean: f64,
    /// The ids of entries scoring below the adequacy threshold on this dimension.
    pub tail_member_ids: Vec<String>,
    /// Fraction of scored entries below threshold. A small share is a genuine
    /// tail (act on `tail_member_ids`); a large share is a systemic deficit.
    pub share_below_threshold: f64,
    /// True when the share crosses [`SYSTEMIC_SHARE`]: the deficit is the
    /// majority, so "tail" understates it and the whole class needs the upgrade.
    pub systemic: bool,
    /// Evidence label describing the strength of this finding.
    pub label: slate_corpus::EvidenceLabel,
}

/// The full result of a gap analysis run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapAnalysis {
    /// The scale this analysis was requested for.
    pub scale: slate_corpus::Scale,
    /// The gap regions discovered across all dimensions.
    pub regions: Vec<GapRegion>,
    /// Tail gap regions discovered across all dimensions.
    pub tail_regions: Vec<TailGapRegion>,
    /// Tier SLA gaps that pertain to the selected entries.
    pub tier_sla_gaps: Vec<slate_tier::TierSlaGap>,
    /// True when neither mean gap regions, tail gap regions, nor tier gaps were found.
    pub null_result: bool,
}

/// The adequacy threshold: dimensions whose mean score falls below this value
/// are reported as gap regions.
const ADEQUACY_THRESHOLD: f64 = 5.0;

/// Share of scored entries below threshold at or above which a dispersion gap is
/// reclassified from a concentrated *tail* to a *systemic* deficit.
const SYSTEMIC_SHARE: f64 = 0.5;

/// Analyze a corpus for gap regions and tier SLA gaps.
///
/// Entries are selected according to `cross_scale`: when `false`, only entries
/// whose scale equals `Some(scale)` are considered; when `true`, every entry in
/// the corpus is considered (cross-scale analysis requires `cross_scale = true`).
///
/// Each selected entry is scored on every [`slate_score::Dimension`] using
/// [`slate_score::ProvisionalScorer::default`]. For each dimension the mean
/// [`slate_score::Score`] value across the selected entries is computed; when it
/// falls below the adequacy threshold a [`GapRegion`] is emitted for that
/// dimension and scale, labelled [`slate_corpus::EvidenceLabel::Provisional`].
///
/// The subset of `tier_gaps` whose `entry_id` is among the selected entry ids is
/// collected. The result's `null_result` is `true` when no regions and no tier
/// gaps were collected.
///
/// The `rubric` parameter is retained for provenance.
pub fn find_gaps(
    corpus: &[slate_corpus::CorpusEntry],
    rubric: &slate_score::Rubric,
    scale: slate_corpus::Scale,
    tier_gaps: &[slate_tier::TierSlaGap],
    cross_scale: bool,
) -> GapAnalysis {
    let _ = rubric;

    let selected: Vec<&slate_corpus::CorpusEntry> = corpus
        .iter()
        .filter(|entry| cross_scale || entry.scale == Some(scale))
        .collect();

    let selected_ids: Vec<String> = selected.iter().map(|entry| entry.id.clone()).collect();

    let scorer = slate_score::ProvisionalScorer::default();

    let mut regions = Vec::new();
    let mut tail_regions = Vec::new();

    for dimension in slate_score::Dimension::all() {
        if selected.is_empty() {
            continue;
        }

        let mut scored: Vec<(&str, f64)> = selected
            .iter()
            .map(|entry| (entry.id.as_str(), scorer.score(entry, dimension).value()))
            .collect();
        let mean_score = scored.iter().map(|(_, value)| value).sum::<f64>() / selected.len() as f64;

        if mean_score < ADEQUACY_THRESHOLD {
            regions.push(GapRegion {
                dimension,
                scale,
                mean_score,
                member_ids: selected_ids.clone(),
                label: slate_corpus::EvidenceLabel::Provisional,
            });
        }

        let under: Vec<String> = scored
            .iter()
            .filter(|(_, value)| *value < ADEQUACY_THRESHOLD)
            .map(|(id, _)| (*id).to_string())
            .collect();
        if !under.is_empty() {
            scored.sort_by(|a, b| a.1.total_cmp(&b.1));
            let quartile = selected.len().div_ceil(4).max(1);
            let tail_mean = scored
                .iter()
                .take(quartile)
                .map(|(_, value)| value)
                .sum::<f64>()
                / quartile as f64;
            if tail_mean < ADEQUACY_THRESHOLD {
                let share = under.len() as f64 / selected.len() as f64;
                tail_regions.push(TailGapRegion {
                    dimension,
                    scale,
                    tail_mean,
                    tail_member_ids: under,
                    share_below_threshold: share,
                    systemic: share >= SYSTEMIC_SHARE,
                    label: slate_corpus::EvidenceLabel::Provisional,
                });
            }
        }
    }

    let tier_sla_gaps: Vec<slate_tier::TierSlaGap> = tier_gaps
        .iter()
        .filter(|gap| selected_ids.iter().any(|id| id == &gap.entry_id))
        .cloned()
        .collect();

    let null_result = regions.is_empty() && tail_regions.is_empty() && tier_sla_gaps.is_empty();

    GapAnalysis {
        scale,
        regions,
        tail_regions,
        tier_sla_gaps,
        null_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_with(
        id: &str,
        score: f64,
        scale: Option<slate_corpus::Scale>,
    ) -> slate_corpus::CorpusEntry {
        let mut scores = std::collections::BTreeMap::new();
        for dim in slate_score::Dimension::all() {
            scores.insert(String::from(dim.code()), score);
        }
        slate_corpus::CorpusEntry {
            id: String::from(id),
            scale,
            scores,
            ..Default::default()
        }
    }

    #[test]
    fn low_scores_yield_gap_region() {
        let corpus = vec![entry_with("A", 2.0, Some(slate_corpus::Scale::National))];
        let rubric = slate_score::Rubric::v0();
        let analysis = find_gaps(&corpus, &rubric, slate_corpus::Scale::National, &[], false);

        assert!(!analysis.regions.is_empty());
        assert!(!analysis.null_result);
        let region = analysis.regions.first().unwrap();
        assert_eq!(region.scale, slate_corpus::Scale::National);
        assert_eq!(region.member_ids, vec![String::from("A")]);
        assert!(matches!(
            region.label,
            slate_corpus::EvidenceLabel::Provisional
        ));
    }

    #[test]
    fn adequate_market_yields_null_result() {
        let corpus = vec![
            entry_with("A", 7.0, Some(slate_corpus::Scale::National)),
            entry_with("B", 9.0, Some(slate_corpus::Scale::National)),
        ];
        let rubric = slate_score::Rubric::v0();
        let analysis = find_gaps(&corpus, &rubric, slate_corpus::Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.tier_sla_gaps.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn split_corpus_flags_tail_gap_even_when_mean_clears_bar() {
        let corpus = vec![
            entry_with("low1", 1.0, Some(slate_corpus::Scale::Regional)),
            entry_with("low2", 1.0, Some(slate_corpus::Scale::Regional)),
            entry_with("high1", 9.0, Some(slate_corpus::Scale::Regional)),
            entry_with("high2", 9.0, Some(slate_corpus::Scale::Regional)),
        ];
        let rubric = slate_score::Rubric::v0();

        let analysis = find_gaps(&corpus, &rubric, slate_corpus::Scale::Regional, &[], false);

        assert!(
            analysis.regions.is_empty(),
            "mean is 5.0, not below the bar"
        );
        assert!(!analysis.tail_regions.is_empty(), "the tail is inadequate");
        assert!(!analysis.null_result);
        let tail = analysis.tail_regions.first().unwrap();
        assert!(tail.tail_mean < 5.0);
        assert!(tail.tail_member_ids.contains(&String::from("low1")));
        assert!(!tail.tail_member_ids.contains(&String::from("high1")));
    }

    #[test]
    fn adequate_market_has_no_tail_gap() {
        let corpus = vec![
            entry_with("A", 7.0, Some(slate_corpus::Scale::National)),
            entry_with("B", 5.0, Some(slate_corpus::Scale::National)),
        ];
        let rubric = slate_score::Rubric::v0();

        let analysis = find_gaps(&corpus, &rubric, slate_corpus::Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn tail_share_classifies_minority_vs_systemic() {
        // 1 under-served of 4 (25%) is a genuine tail; 3 of 4 (75%) is systemic.
        let minority = vec![
            entry_with("low1", 1.0, Some(slate_corpus::Scale::Regional)),
            entry_with("hi1", 9.0, Some(slate_corpus::Scale::Regional)),
            entry_with("hi2", 9.0, Some(slate_corpus::Scale::Regional)),
            entry_with("hi3", 9.0, Some(slate_corpus::Scale::Regional)),
        ];
        let rubric = slate_score::Rubric::v0();
        let a = find_gaps(
            &minority,
            &rubric,
            slate_corpus::Scale::Regional,
            &[],
            false,
        );
        let tail = a.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.25).abs() < 1e-9);
        assert!(!tail.systemic);

        let majority = vec![
            entry_with("low1", 1.0, Some(slate_corpus::Scale::Regional)),
            entry_with("low2", 1.0, Some(slate_corpus::Scale::Regional)),
            entry_with("low3", 1.0, Some(slate_corpus::Scale::Regional)),
            entry_with("hi1", 9.0, Some(slate_corpus::Scale::Regional)),
        ];
        let b = find_gaps(
            &majority,
            &rubric,
            slate_corpus::Scale::Regional,
            &[],
            false,
        );
        let tail = b.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.75).abs() < 1e-9);
        assert!(tail.systemic);
    }

    #[test]
    fn other_scale_excluded_unless_cross_scale() {
        let corpus = vec![entry_with("A", 2.0, Some(slate_corpus::Scale::Regional))];
        let rubric = slate_score::Rubric::v0();

        let excluded = find_gaps(&corpus, &rubric, slate_corpus::Scale::National, &[], false);
        assert!(excluded.regions.is_empty());
        assert!(excluded.null_result);

        let included = find_gaps(&corpus, &rubric, slate_corpus::Scale::National, &[], true);
        assert!(!included.regions.is_empty());
        assert!(!included.null_result);
    }
}
