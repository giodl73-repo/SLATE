use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use slate_corpus::CorpusEntry;

/// One of the thirteen SLATE scoring dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Dimension {
    Dim01,
    Dim02,
    Dim03,
    Dim04,
    Dim05,
    Dim06,
    Dim07,
    Dim08,
    Dim09,
    Dim10,
    Dim11,
    Dim12,
    Dim13,
}

impl Dimension {
    /// Stable textual code for the dimension, e.g. `DIM-01`.
    pub fn code(&self) -> &'static str {
        match self {
            Dimension::Dim01 => "DIM-01",
            Dimension::Dim02 => "DIM-02",
            Dimension::Dim03 => "DIM-03",
            Dimension::Dim04 => "DIM-04",
            Dimension::Dim05 => "DIM-05",
            Dimension::Dim06 => "DIM-06",
            Dimension::Dim07 => "DIM-07",
            Dimension::Dim08 => "DIM-08",
            Dimension::Dim09 => "DIM-09",
            Dimension::Dim10 => "DIM-10",
            Dimension::Dim11 => "DIM-11",
            Dimension::Dim12 => "DIM-12",
            Dimension::Dim13 => "DIM-13",
        }
    }

    /// All thirteen dimensions, in canonical order.
    pub fn all() -> [Dimension; 13] {
        [
            Dimension::Dim01,
            Dimension::Dim02,
            Dimension::Dim03,
            Dimension::Dim04,
            Dimension::Dim05,
            Dimension::Dim06,
            Dimension::Dim07,
            Dimension::Dim08,
            Dimension::Dim09,
            Dimension::Dim10,
            Dimension::Dim11,
            Dimension::Dim12,
            Dimension::Dim13,
        ]
    }
}

/// A bounded score in the inclusive range `0.0..=10.0`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Score(f64);

impl Score {
    const MIN: f64 = 0.0;
    const MAX: f64 = 10.0;

    /// Construct a `Score`, returning `None` if `value` is outside `0.0..=10.0`.
    pub fn new(value: f64) -> Option<Score> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            None
        } else {
            Some(Score(value))
        }
    }

    /// Construct a `Score`, clamping `value` into `0.0..=10.0`.
    pub fn clamped(value: f64) -> Score {
        Score(value.clamp(Self::MIN, Self::MAX))
    }

    /// The underlying floating-point value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A weighted rubric over the scoring dimensions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rubric {
    pub version: String,
    pub weights: BTreeMap<Dimension, f64>,
}

impl Rubric {
    /// The provisional `v0` rubric: every dimension weighted equally at `1.0`.
    pub fn v0() -> Rubric {
        let mut weights = BTreeMap::new();
        for dimension in Dimension::all() {
            weights.insert(dimension, 1.0);
        }
        Rubric {
            version: "v0".to_string(),
            weights,
        }
    }

    /// A short, human-readable explanation of the rubric.
    pub fn rationale(&self) -> &'static str {
        "Provisional v0 rubric: all thirteen dimensions weighted equally."
    }
}

/// Produces a `Score` for a given corpus entry and dimension.
pub trait DimensionScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Score;
}

/// A baseline scorer that reads precomputed scores from a corpus entry.
#[derive(Clone, Debug)]
pub struct ProvisionalScorer {
    pub rubric: Rubric,
}

impl Default for ProvisionalScorer {
    fn default() -> Self {
        ProvisionalScorer {
            rubric: Rubric::v0(),
        }
    }
}

impl DimensionScorer for ProvisionalScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Score {
        let raw = entry.scores.get(dimension.code()).copied().unwrap_or(0.0);
        Score::clamped(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_new_rejects_above_max() {
        assert!(Score::new(10.000_1).is_none());
    }

    #[test]
    fn score_new_rejects_below_min() {
        assert!(Score::new(-0.000_1).is_none());
    }

    #[test]
    fn score_new_accepts_in_range() {
        assert!(Score::new(0.0).is_some());
        assert!(Score::new(10.0).is_some());
        let mid = Score::new(5.5).unwrap();
        assert_eq!(mid.value(), 5.5);
    }

    #[test]
    fn score_clamped_maps_out_of_range() {
        assert_eq!(Score::clamped(-3.0).value(), 0.0);
        assert_eq!(Score::clamped(42.0).value(), 10.0);
        assert_eq!(Score::clamped(7.25).value(), 7.25);
    }

    #[test]
    fn rubric_v0_has_expected_shape() {
        let rubric = Rubric::v0();
        assert_eq!(rubric.version, "v0");
        for dimension in Dimension::all() {
            assert_eq!(rubric.weights.get(&dimension), Some(&1.0));
        }
        assert_eq!(rubric.weights.len(), 13);
        assert!(!rubric.rationale().is_empty());
    }

    #[test]
    fn provisional_scorer_reads_present_score() {
        let mut entry = CorpusEntry::default();
        entry
            .scores
            .insert(Dimension::Dim03.code().to_string(), 6.5);

        let scorer = ProvisionalScorer::default();
        let score = scorer.score(&entry, Dimension::Dim03);
        assert_eq!(score.value(), 6.5);
    }

    #[test]
    fn provisional_scorer_defaults_absent_to_zero() {
        let entry = CorpusEntry::default();
        let scorer = ProvisionalScorer::default();
        let score = scorer.score(&entry, Dimension::Dim07);
        assert_eq!(score.value(), 0.0);
    }

    #[test]
    fn provisional_scorer_bounds_out_of_range_values() {
        let mut entry = CorpusEntry::default();
        entry
            .scores
            .insert(Dimension::Dim01.code().to_string(), 99.0);

        let scorer = ProvisionalScorer::default();
        let score = scorer.score(&entry, Dimension::Dim01);
        assert_eq!(score.value(), 10.0);
    }
}
