use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scale {
    International,
    National,
    Regional,
    Local,
}

impl Scale {
    pub fn parse(s: &str) -> Option<Scale> {
        match s {
            "international" => Some(Scale::International),
            "national" => Some(Scale::National),
            "regional" => Some(Scale::Regional),
            "local" => Some(Scale::Local),
            _ => None,
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Scale::International => "international",
            Scale::National => "national",
            Scale::Regional => "regional",
            Scale::Local => "local",
        };
        f.write_str(s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLabel {
    Estimated,
    Cited,
    Validated,
    Provisional,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    Surge,
    Baseline,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub label: EvidenceLabel,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusEntry {
    pub id: String,
    pub kind: String,
    pub scale: Option<Scale>,
    pub jurisdiction: String,
    pub tier: Option<String>,
    pub sla: Option<String>,
    pub quantities: Vec<Quantity>,
    pub scores: BTreeMap<String, f64>,
}

#[derive(Debug, Error)]
pub enum CorpusError {
    #[error("corpus entry is missing a required id")]
    MissingId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoldReason {
    UncitedQuantity(String),
    MissingScale,
}

impl CorpusEntry {
    pub fn validate(&self) -> Result<Vec<HoldReason>, CorpusError> {
        if self.id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        let mut reasons = Vec::new();

        if self.scale.is_none() {
            reasons.push(HoldReason::MissingScale);
        }

        for quantity in &self.quantities {
            if quantity.source_id.is_none() {
                reasons.push(HoldReason::UncitedQuantity(quantity.unit.clone()));
            }
        }

        Ok(reasons)
    }

    pub fn from_markdown(input: &str) -> Result<CorpusEntry, CorpusError> {
        let mut id = String::new();
        let mut kind = String::new();
        let mut scale: Option<Scale> = None;
        let mut jurisdiction = String::new();
        let mut tier: Option<String> = None;

        let mut lines = input.lines();
        let mut in_frontmatter = false;

        for line in lines.by_ref() {
            let trimmed = line.trim();
            if trimmed == "---" {
                if in_frontmatter {
                    break;
                }
                in_frontmatter = true;
                continue;
            }

            if !in_frontmatter {
                continue;
            }

            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim();
                let value = value.trim().to_string();
                match key {
                    "id" => id = value,
                    "kind" | "type" => kind = value,
                    "scale" => scale = Scale::parse(&value),
                    "jurisdiction" => jurisdiction = value,
                    "tier" => {
                        if value.is_empty() {
                            tier = None;
                        } else {
                            tier = Some(value);
                        }
                    }
                    _ => {}
                }
            }
        }

        if id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        Ok(CorpusEntry {
            id,
            kind,
            scale,
            jurisdiction,
            tier,
            sla: None,
            quantities: Vec::new(),
            scores: BTreeMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_id_rejected() {
        let entry = CorpusEntry::default();
        match entry.validate() {
            Err(CorpusError::MissingId) => {}
            other => panic!("expected MissingId, got {:?}", other),
        }
    }

    #[test]
    fn none_scale_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::MissingScale));
    }

    #[test]
    fn uncited_quantity_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::National),
            quantities: vec![Quantity {
                value: 10.0,
                unit: "beds".to_string(),
                label: EvidenceLabel::Estimated,
                source_id: None,
            }],
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::UncitedQuantity("beds".to_string())));
    }

    #[test]
    fn evidence_label_unchanged_after_validate() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::National),
            quantities: vec![Quantity {
                value: 10.0,
                unit: "beds".to_string(),
                label: EvidenceLabel::Validated,
                source_id: None,
            }],
            ..Default::default()
        };
        let _ = entry.validate().unwrap();
        assert_eq!(entry.quantities[0].label, EvidenceLabel::Validated);
    }

    #[test]
    fn from_markdown_parses_frontmatter() {
        let input =
            "---\nid: e1\nkind: facility\nscale: national\njurisdiction: us\n---\nbody text\n";
        let entry = CorpusEntry::from_markdown(input).unwrap();
        assert_eq!(entry.id, "e1");
        assert_eq!(entry.scale, Some(Scale::National));
    }

    #[test]
    fn from_markdown_missing_id_rejected() {
        let input = "---\nkind: facility\nscale: national\n---\n";
        match CorpusEntry::from_markdown(input) {
            Err(CorpusError::MissingId) => {}
            other => panic!("expected MissingId, got {:?}", other),
        }
    }
}
