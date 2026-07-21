use std::collections::{HashMap, HashSet};

use petgraph::algo::{astar, has_path_connecting};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::NodeFiltered;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchoolRole {
    Elementary,
    Secondary,
    Postsecondary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    Surge,
    Baseline,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct School {
    pub id: String,
    pub name: String,
    pub role: SchoolRole,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pathway {
    pub id: String,
    pub capacity_seats: f64,
    pub basis: DemandBasis,
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("duplicate school id: {0}")]
    DuplicateSchool(String),
    #[error("unknown school id: {0}")]
    UnknownSchool(String),
    #[error("non-positive capacity: {0}")]
    NonPositiveCapacity(f64),
}

pub struct Network {
    graph: UnGraph<School, Pathway>,
    index: HashMap<String, NodeIndex>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            graph: UnGraph::new_undirected(),
            index: HashMap::new(),
        }
    }

    pub fn add_school(&mut self, school: School) -> Result<(), NetworkError> {
        if self.index.contains_key(&school.id) {
            return Err(NetworkError::DuplicateSchool(school.id));
        }
        let id = school.id.clone();
        let idx = self.graph.add_node(school);
        self.index.insert(id, idx);
        Ok(())
    }

    pub fn add_pathway(
        &mut self,
        from_id: &str,
        to_id: &str,
        pathway: Pathway,
    ) -> Result<(), NetworkError> {
        if pathway.capacity_seats <= 0.0 {
            return Err(NetworkError::NonPositiveCapacity(pathway.capacity_seats));
        }
        let from = *self
            .index
            .get(from_id)
            .ok_or_else(|| NetworkError::UnknownSchool(from_id.to_string()))?;
        let to = *self
            .index
            .get(to_id)
            .ok_or_else(|| NetworkError::UnknownSchool(to_id.to_string()))?;
        self.graph.add_edge(from, to, pathway);
        Ok(())
    }

    pub fn school_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn pathway_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn degree(&self, id: &str) -> Option<usize> {
        self.index.get(id).map(|&idx| self.graph.edges(idx).count())
    }

    pub fn is_connected(&self, a: &str, b: &str) -> bool {
        match (self.index.get(a), self.index.get(b)) {
            (Some(&ai), Some(&bi)) => has_path_connecting(&self.graph, ai, bi, None),
            _ => false,
        }
    }

    pub fn has_diverse_path(&self, a: &str, b: &str) -> bool {
        let (ai, bi) = match (self.index.get(a), self.index.get(b)) {
            (Some(&ai), Some(&bi)) => (ai, bi),
            _ => return false,
        };
        if ai == bi {
            return false;
        }
        let path = match astar(&self.graph, ai, |n| n == bi, |_| 1i32, |_| 0i32) {
            Some((_, p)) => p,
            None => return false,
        };
        if path.len() < 2 {
            return false;
        }
        let blocked: HashSet<NodeIndex> = path[1..path.len() - 1].iter().copied().collect();
        let filtered = NodeFiltered::from_fn(&self.graph, |n| !blocked.contains(&n));
        has_path_connecting(&filtered, ai, bi, None)
    }

    pub fn incident_capacity_seats(&self, id: &str) -> f64 {
        let idx = match self.index.get(id) {
            Some(&idx) => idx,
            None => return 0.0,
        };
        self.graph
            .edge_indices()
            .filter_map(|e| self.graph.edge_endpoints(e).map(|ends| (e, ends)))
            .filter(|&(_, (s, t))| s == idx || t == idx)
            .filter_map(|(e, _)| self.graph.edge_weight(e))
            .map(|p| p.capacity_seats)
            .sum()
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn school(id: &str, name: &str, role: SchoolRole) -> School {
        School {
            id: id.to_string(),
            name: name.to_string(),
            role,
        }
    }

    fn pathway(id: &str, capacity_seats: f64, basis: DemandBasis) -> Pathway {
        Pathway {
            id: id.to_string(),
            capacity_seats,
            basis,
        }
    }

    fn sample() -> Network {
        let mut net = Network::new();
        net.add_school(school("A", "Alpha", SchoolRole::Elementary))
            .unwrap();
        net.add_school(school("B", "Beta", SchoolRole::Secondary))
            .unwrap();
        net.add_school(school("C", "Gamma", SchoolRole::Postsecondary))
            .unwrap();
        net.add_pathway("A", "B", pathway("AB", 10.0, DemandBasis::Surge))
            .unwrap();
        net.add_pathway("B", "C", pathway("BC", 20.0, DemandBasis::Baseline))
            .unwrap();
        net
    }

    #[test]
    fn counts_and_degree() {
        let net = sample();
        assert_eq!(net.school_count(), 3);
        assert_eq!(net.pathway_count(), 2);
        assert_eq!(net.degree("B"), Some(2));
        assert_eq!(net.degree("A"), Some(1));
        assert_eq!(net.degree("Z"), None);
    }

    #[test]
    fn connectivity() {
        let mut net = sample();
        assert!(net.is_connected("A", "C"));
        net.add_school(school("D", "Delta", SchoolRole::Elementary))
            .unwrap();
        assert!(!net.is_connected("A", "D"));
    }

    #[test]
    fn incident_capacity_sums() {
        let net = sample();
        assert!((net.incident_capacity_seats("B") - 30.0).abs() < 1e-9);
        assert!((net.incident_capacity_seats("A") - 10.0).abs() < 1e-9);
        assert!((net.incident_capacity_seats("C") - 20.0).abs() < 1e-9);
    }

    #[test]
    fn basis_is_preserved() {
        let net = sample();
        let surge = net
            .graph
            .edge_indices()
            .filter_map(|e| net.graph.edge_weight(e))
            .find(|p| p.id == "AB")
            .map(|p| p.basis);
        let baseline = net
            .graph
            .edge_indices()
            .filter_map(|e| net.graph.edge_weight(e))
            .find(|p| p.id == "BC")
            .map(|p| p.basis);
        assert_eq!(surge, Some(DemandBasis::Surge));
        assert_eq!(baseline, Some(DemandBasis::Baseline));
    }

    #[test]
    fn diverse_path_on_ring_but_not_chain() {
        let mut ring = Network::new();
        ring.add_school(school("A", "A", SchoolRole::Elementary))
            .unwrap();
        ring.add_school(school("B", "B", SchoolRole::Elementary))
            .unwrap();
        ring.add_school(school("C", "C", SchoolRole::Elementary))
            .unwrap();
        ring.add_school(school("D", "D", SchoolRole::Elementary))
            .unwrap();
        ring.add_pathway("A", "B", pathway("AB", 1.0, DemandBasis::Baseline))
            .unwrap();
        ring.add_pathway("B", "C", pathway("BC", 1.0, DemandBasis::Baseline))
            .unwrap();
        ring.add_pathway("C", "D", pathway("CD", 1.0, DemandBasis::Baseline))
            .unwrap();
        ring.add_pathway("D", "A", pathway("DA", 1.0, DemandBasis::Baseline))
            .unwrap();
        assert!(ring.has_diverse_path("A", "C"));

        let chain = sample();
        assert!(!chain.has_diverse_path("A", "C"));
    }

    #[test]
    fn rejects_duplicate_school() {
        let mut net = Network::new();
        net.add_school(school("A", "Alpha", SchoolRole::Elementary))
            .unwrap();
        let err = net
            .add_school(school("A", "Other", SchoolRole::Secondary))
            .unwrap_err();
        assert!(matches!(err, NetworkError::DuplicateSchool(_)));
    }

    #[test]
    fn rejects_bad_pathway() {
        let mut net = Network::new();
        net.add_school(school("A", "Alpha", SchoolRole::Elementary))
            .unwrap();
        net.add_school(school("B", "Beta", SchoolRole::Secondary))
            .unwrap();

        let bad_cap = net
            .add_pathway("A", "B", pathway("AB", 0.0, DemandBasis::Surge))
            .unwrap_err();
        assert!(matches!(bad_cap, NetworkError::NonPositiveCapacity(_)));

        let unknown = net
            .add_pathway("A", "Z", pathway("AZ", 5.0, DemandBasis::Surge))
            .unwrap_err();
        assert!(matches!(unknown, NetworkError::UnknownSchool(_)));
    }
}
