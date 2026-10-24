//! Frozen, public state aggregates. No seat, class-size or access inference.
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

const DATA: &str = include_str!("../../../fixtures/nces/summary.json");
const FIELDS: [(&str, &str); 5] = [
    ("operating_schools", "schools"),
    ("operating_districts", "districts"),
    ("student_membership", "students"),
    ("teacher_fte", "published_integer_fte"),
    ("pupil_teacher_ratio", "students_per_teacher_fte_tenths"),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    view: View,
    jurisdiction: String,
}

#[derive(Deserialize)]
enum View {
    #[serde(rename = "nces-state-comparison")]
    Comparison,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Dataset {
    schema: String,
    captured_at: String,
    releases: Vec<Release>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Release {
    school_year: String,
    source_id: String,
    url: String,
    sha256: String,
    notes: Vec<String>,
    rows: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    jurisdiction: String,
    group: String,
    operating_schools: String,
    operating_districts: String,
    student_membership: String,
    teacher_fte: String,
    pupil_teacher_ratio: String,
}

impl Row {
    fn cells(&self) -> [&str; 5] {
        [
            &self.operating_schools,
            &self.operating_districts,
            &self.student_membership,
            &self.teacher_fte,
            &self.pupil_teacher_ratio,
        ]
    }
}

fn number(raw: &str, ratio: bool) -> Result<Option<i64>, String> {
    if matches!(raw, "—" | "‡") {
        return Ok(None);
    }
    if ratio {
        let (whole, decimal) = raw
            .split_once('.')
            .ok_or("Ratio requires published tenth precision")?;
        if decimal.len() != 1 || !decimal.bytes().all(|c| c.is_ascii_digit()) {
            return Err("Invalid ratio precision".into());
        }
        let whole = unsigned(whole)?;
        let tenth = unsigned(decimal)?;
        if whole > 1000 {
            return Err("Ratio outside source bounds".into());
        }
        return Ok(Some(whole * 10 + tenth));
    }
    let digits = raw.replace(',', "");
    let value = unsigned(&digits)?;
    // Commas, when present, must use the source's three-digit grouping.
    if raw.contains(',') {
        let groups: Vec<&str> = raw.split(',').collect();
        if groups[0].is_empty() || groups[0].len() > 3 || groups[1..].iter().any(|s| s.len() != 3) {
            return Err("Invalid source thousands grouping".into());
        }
    }
    Ok(Some(value))
}

fn unsigned(raw: &str) -> Result<i64, String> {
    if raw.is_empty() || raw.len() > 10 || !raw.bytes().all(|c| c.is_ascii_digit()) {
        return Err("Invalid published nonnegative number".into());
    }
    let value = raw.parse::<i64>().map_err(|error| error.to_string())?;
    if value > 1_000_000_000 {
        return Err("Source count outside bound".into());
    }
    Ok(value)
}

fn validate(data: &Dataset) -> Result<(), String> {
    if data.schema != "slate.nces-table2.v1" || data.releases.len() != 2 {
        return Err("Unexpected frozen source schema".into());
    }
    let mut previous = None;
    for (release, year) in data.releases.iter().zip(["2023-24", "2024-25"]) {
        if release.school_year != year
            || release.rows.len() != 59
            || release.sha256.len() != 64
            || !release.sha256.bytes().all(|c| c.is_ascii_hexdigit())
            || !release.url.starts_with("https://nces.ed.gov/ccd/tables/")
            || !release
                .notes
                .iter()
                .any(|note| note.contains("Final Version 1a"))
        {
            return Err("Unexpected source identity or scope".into());
        }
        let mut names = BTreeSet::new();
        let mut unique_names = BTreeSet::new();
        for row in &release.rows {
            if row.jurisdiction.is_empty()
                || !unique_names.insert(&row.jurisdiction)
                || !names.insert((&row.jurisdiction, &row.group))
            {
                return Err("Duplicate or empty jurisdiction".into());
            }
            if !matches!(
                row.group.as_str(),
                "state_dc" | "us_total" | "other_jurisdiction"
            ) {
                return Err("Unknown source population".into());
            }
            for (index, raw) in row.cells().iter().enumerate() {
                number(raw, index == 4)?;
            }
        }
        if release
            .rows
            .iter()
            .filter(|row| row.group == "state_dc")
            .count()
            != 51
            || release
                .rows
                .iter()
                .filter(|row| row.group == "other_jurisdiction")
                .count()
                != 7
            || release
                .rows
                .iter()
                .filter(|row| row.group == "us_total" && row.jurisdiction == "United States")
                .count()
                != 1
        {
            return Err(
                "Source populations do not match 50 states plus DC and separate other rows".into(),
            );
        }
        if previous.as_ref().is_some_and(|prior| prior != &names) {
            return Err("Unmatched jurisdiction populations".into());
        }
        previous = Some(names);
    }
    Ok(())
}

fn observation(raw: &str, unit: &str, ratio: bool) -> Result<Value, String> {
    let value = number(raw, ratio)?;
    let status = match raw {
        "—" => "not_available",
        "‡" => "withheld_see_source_note",
        _ => "reported",
    };
    Ok(json!({"published":raw,"value":value,"unit":unit,"status":status}))
}

pub fn evaluate(input: &str) -> Result<String, String> {
    let query: Query = serde_json::from_str(input).map_err(|error| error.to_string())?;
    let View::Comparison = query.view;
    let data: Dataset = serde_json::from_str(DATA).map_err(|error| error.to_string())?;
    validate(&data)?;
    let mut observations = Vec::new();
    let mut sources = Vec::new();
    let mut coverage = Vec::new();
    let mut selected_rows = Vec::new();
    for release in &data.releases {
        let selected = release
            .rows
            .iter()
            .find(|row| row.jurisdiction == query.jurisdiction)
            .ok_or("Choose a published jurisdiction")?;
        let national = release
            .rows
            .iter()
            .find(|row| row.group == "us_total")
            .ok_or("Missing US source row")?;
        let mut values = Map::new();
        let mut field_coverage = Map::new();
        for (index, ((key, unit), raw)) in FIELDS.iter().zip(selected.cells()).enumerate() {
            values.insert((*key).into(), observation(raw, unit, index == 4)?);
            if index == 4 {
                continue;
            } // Ratios are not additive.
            let mut sum = 0;
            let mut missing = Vec::new();
            for row in release.rows.iter().filter(|row| row.group == "state_dc") {
                match number(row.cells()[index], false)? {
                    Some(value) => sum += value,
                    None => missing.push(&row.jurisdiction),
                }
            }
            let complete_sum = missing.is_empty().then_some(sum);
            let national_value = number(national.cells()[index], false)?;
            let residual = complete_sum
                .zip(national_value)
                .map(|(sum, published)| published - sum);
            field_coverage.insert((*key).into(), json!({"state_dc_rows":51,"reported_rows":51-missing.len(),"missing_jurisdictions":missing,"complete_state_dc_sum":complete_sum,"published_us_total":national_value,"published_minus_complete_sum":residual}));
        }
        observations.push(json!({"school_year":release.school_year,"source_id":release.source_id,"values":values}));
        coverage.push(json!({"school_year":release.school_year,"fields":field_coverage}));
        sources.push(json!({"school_year":release.school_year,"source_id":release.source_id,"url":release.url,"sha256":release.sha256,"notes":release.notes}));
        selected_rows.push(selected);
    }
    let mut changes = Map::new();
    for (index, (key, unit)) in FIELDS.iter().enumerate() {
        let before = number(selected_rows[0].cells()[index], index == 4)?;
        let after = number(selected_rows[1].cells()[index], index == 4)?;
        changes.insert(
            (*key).into(),
            json!({"value":after.zip(before).map(|(a,b)|a-b),"unit":unit}),
        );
    }
    serde_json::to_string(&json!({
        "schema":"slate.nces-state-comparison.v1","evidence":"frozen_official_public_aggregate",
        "captured_at":data.captured_at,"jurisdiction":query.jurisdiction,"population_group":selected_rows[0].group,
        "scope":{"us_total":"50 states plus DC only","state_dc":"publisher state aggregate; not school, program or student records","other_jurisdiction":"separate territories/agency rows; excluded from US totals","grade_scope":"Pre-K through grade 13; adult education excluded","teacher_precision":"Published integer FTE, not headcount or microdata precision"},
        "observations":observations,"changes_2024_25_minus_2023_24":changes,"state_dc_coverage":coverage,
        "jurisdictions":data.releases[0].rows.iter().map(|row|json!({"name":row.jurisdiction,"group":row.group})).collect::<Vec<_>>(),"sources":sources,
        "not_assessed":{"class_size":null,"usable_staffed_seats":null,"grade_program_language_support_fit":null,"eligible_pathways":null,"learning_outcomes":null,"actual_access_equity":null,"affordability":null,"funding_governance":null},
        "authority":{"assignment_admissions":false,"funding_curriculum":false,"boundary_labor":false,"allocation":false,"savings":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(name: &str) -> Value {
        serde_json::from_str(
            &evaluate(&json!({"view":"nces-state-comparison","jurisdiction":name}).to_string())
                .unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn national_missing_staff_cannot_be_reconstructed_from_partial_states() {
        let v = run("United States");
        assert_eq!(
            v["observations"][0]["values"]["student_membership"]["value"],
            49_404_386
        );
        assert_eq!(
            v["changes_2024_25_minus_2023_24"]["student_membership"]["value"],
            -134_504
        );
        assert!(v["observations"][1]["values"]["teacher_fte"]["value"].is_null());
        assert!(v["changes_2024_25_minus_2023_24"]["teacher_fte"]["value"].is_null());
        assert!(
            v["state_dc_coverage"][1]["fields"]["teacher_fte"]["complete_state_dc_sum"].is_null()
        );
        assert_eq!(
            v["state_dc_coverage"][1]["fields"]["teacher_fte"]["missing_jurisdictions"],
            json!(["Tennessee"])
        );
    }
    #[test]
    fn matched_state_changes_and_published_ratio_remain_separate_from_class_size() {
        let v = run("California");
        assert_eq!(
            v["changes_2024_25_minus_2023_24"]["teacher_fte"]["value"],
            -251
        );
        assert_eq!(
            v["changes_2024_25_minus_2023_24"]["pupil_teacher_ratio"]["value"],
            -1
        );
        assert!(v["not_assessed"]["class_size"].is_null());
        assert_eq!(v["authority"]["allocation"], false);
    }
    #[test]
    fn source_withholding_and_unavailability_are_distinct() {
        let t = run("Tennessee");
        assert_eq!(
            t["observations"][1]["values"]["teacher_fte"]["published"],
            "—"
        );
        let g = run("Guam");
        assert_eq!(g["population_group"], "other_jurisdiction");
        assert_eq!(
            g["observations"][0]["values"]["student_membership"]["status"],
            "not_available"
        );
        assert_eq!(
            g["observations"][1]["values"]["student_membership"]["status"],
            "withheld_see_source_note"
        );
    }
    #[test]
    fn malformed_source_numbers_and_queries_fail_closed() {
        for raw in ["-1", "1,2", "NaN", "", "1e4"] {
            assert!(number(raw, false).is_err());
        }
        assert!(number("15.22", true).is_err());
        assert!(evaluate(
            r#"{"view":"nces-state-comparison","jurisdiction":"California","student_name":"x"}"#
        )
        .is_err());
        assert!(evaluate(r#"{"view":"nces-state-comparison","jurisdiction":"XX"}"#).is_err());
    }
}
