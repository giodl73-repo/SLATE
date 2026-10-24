//! Fictional school topology, not staffed instruction or eligible progression.
use serde::{Deserialize, Serialize};
use slate_network::{DemandBasis, Network, Pathway, School, SchoolRole};
use std::collections::HashSet;

pub const MAX_INPUT_BYTES: usize = 65_536;

mod public_source;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub id: String,
    pub from: String,
    pub to: String,
    pub capacity_seats: f64,
    pub basis: DemandBasis,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Site {
    pub id: String,
    pub name: String,
    pub role: SchoolRole,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub scale: String,
    pub jurisdiction: String,
    pub comparison_window: String,
    pub selected_node: String,
    pub destination: String,
    pub nodes: Vec<Site>,
    pub links: Vec<Link>,
}

fn safe_id(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 64
        && text
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c))
}

/// Evaluate a bounded native graph task or a frozen public-source query.
pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of bounded task JSON".into());
    }
    let envelope: serde_json::Value =
        serde_json::from_str(input).map_err(|error| error.to_string())?;
    if envelope.get("view").is_some() {
        return public_source::evaluate(input);
    }
    evaluate_graph(input)
}

fn evaluate_graph(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of synthetic graph JSON".into());
    }
    let input: Input = serde_json::from_str(input).map_err(|error| error.to_string())?;
    if input.scale != "local"
        || input.jurisdiction != "fictional-cedar"
        || input.comparison_window != "synthetic-surge-enrollment"
    {
        return Err("Keep the local fictional Cedar scope and common surge basis".into());
    }
    if input.nodes.is_empty() || input.nodes.len() > 16 || input.links.len() > 32 {
        return Err("Use 1–16 nodes and at most 32 links".into());
    }
    let mut graph = Network::new();
    for site in &input.nodes {
        if !safe_id(&site.id) || !safe_id(&site.name) {
            return Err("Use short fictional identifiers for nodes, names and kinds".into());
        }
        graph
            .add_school(School {
                id: site.id.clone(),
                name: site.name.clone(),
                role: site.role,
            })
            .map_err(|error| error.to_string())?;
    }
    let mut links = HashSet::new();
    let mut pairs = HashSet::new();
    for link in &input.links {
        let pair = if link.from < link.to {
            (&link.from, &link.to)
        } else {
            (&link.to, &link.from)
        };
        if !pairs.insert(pair) {
            return Err("Use one illustrative pathway per unordered school pair".into());
        }
        if !safe_id(&link.id)
            || !links.insert(&link.id)
            || link.from == link.to
            || !input.nodes.iter().any(|node| node.id == link.from)
            || !input.nodes.iter().any(|node| node.id == link.to)
        {
            return Err("Each link needs a unique safe id and two distinct known nodes".into());
        }
        if !link.capacity_seats.is_finite()
            || link.capacity_seats <= 0.0
            || link.capacity_seats > 1_000_000_000.0
        {
            return Err("Link ratings must be finite, positive and at most 1 billion seats".into());
        }
        if link.basis != DemandBasis::Surge {
            return Err("Mixed or baseline basis is not comparable in this surge example".into());
        }
        if link.enabled {
            graph
                .add_pathway(
                    &link.from,
                    &link.to,
                    Pathway {
                        id: link.id.clone(),
                        capacity_seats: link.capacity_seats,
                        basis: link.basis,
                    },
                )
                .map_err(|error| error.to_string())?;
        }
    }
    if !input
        .nodes
        .iter()
        .any(|site| site.id == input.destination && site.role == SchoolRole::Postsecondary)
    {
        return Err("Name a known Postsecondary target".into());
    }
    let degree = graph
        .degree(&input.selected_node)
        .ok_or_else(|| "Unknown selected school".to_string())?;
    let incident = graph.incident_capacity_seats(&input.selected_node);
    let connected = graph.is_connected(&input.selected_node, &input.destination);
    // Canonical object keys are sorted by serde_json's default map implementation.
    serde_json::to_string(&serde_json::json!({
        "schema":"slate.synthetic-local-connectivity.v1", "evidence":"simulated",
        "scope":{"scale":input.scale,"jurisdiction":input.jurisdiction,"window":input.comparison_window,"unit":"seats"},
        "selected_node":input.selected_node,"destination":input.destination,
        "metrics":{"enabled_incident_pathways":degree,"incident_capacity_seats":incident,"path_connected_to_destination":connected},
        "nodes":input.nodes,"links":input.links,
        "not_assessed":{"staffed_instruction":null,"grade_program_language_support_fit":null,"transfer_eligibility_credit_acceptance":null,"learning_attainment_mobility":null,"affordability":null,"equity_access":null,"funding_boundaries_governance":null},
        "authority":{"assignment_admissions":false,"funding_curriculum":false,"boundary_labor":false,"allocation":false,"savings":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

pub fn response(input: &str) -> String {
    match evaluate(input) {
        Ok(canonical) => serde_json::json!({"ok":true,"canonical":canonical,"result":serde_json::from_str::<serde_json::Value>(&canonical).expect("serialized result")}).to_string(),
        Err(error) => serde_json::json!({"ok":false,"error":error}).to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
mod abi;

#[cfg(test)]
mod tests {
    use super::*;
    const RESET: &str = include_str!("../../../fixtures/browser-reset.json");
    #[test]
    fn native_incident_ratings_and_connectivity_are_separate() {
        let v: serde_json::Value = serde_json::from_str(&evaluate(RESET).unwrap()).unwrap();
        assert_eq!(v["metrics"]["incident_capacity_seats"], 120.0);
        assert_eq!(v["metrics"]["path_connected_to_destination"], true);
        assert!(v["not_assessed"]["staffed_instruction"].is_null());
    }
    #[test]
    fn more_adjacent_seats_cannot_reconnect_a_broken_handoff() {
        let mut input: serde_json::Value = serde_json::from_str(RESET).unwrap();
        input["links"][0]["capacity_seats"] = serde_json::json!(240);
        input["links"][1]["enabled"] = serde_json::json!(false);
        let v: serde_json::Value =
            serde_json::from_str(&evaluate(&input.to_string()).unwrap()).unwrap();
        assert_eq!(v["metrics"]["incident_capacity_seats"], 240.0);
        assert_eq!(v["metrics"]["path_connected_to_destination"], false);
    }
    #[test]
    fn rejects_mixed_basis_unknown_target_and_nonpositive_rating() {
        assert!(evaluate(&RESET.replace("Surge", "Baseline")).is_err());
        let mut input: serde_json::Value = serde_json::from_str(RESET).unwrap();
        input["destination"] = serde_json::json!("community");
        assert!(evaluate(&input.to_string()).is_err());
        input = serde_json::from_str(RESET).unwrap();
        input["links"][0]["capacity_seats"] = serde_json::json!(0);
        assert!(evaluate(&input.to_string()).is_err());
        assert!(evaluate(&"x".repeat(65537)).is_err());
    }
}
