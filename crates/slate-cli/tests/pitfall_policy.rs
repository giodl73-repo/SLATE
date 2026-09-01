use serde_json::Value;

fn public_claim_boundaries() -> Value {
    serde_json::from_str(include_str!(
        "../../../docs/public-claim-boundaries.v1.json"
    ))
    .expect("public claim boundary manifest must remain valid JSON")
}

fn pitfall_boundary<'a>(manifest: &'a Value, pitfall: &str) -> &'a Value {
    manifest["pitfall_boundaries"]
        .as_array()
        .expect("manifest should retain pitfall boundaries")
        .iter()
        .find(|boundary| boundary["pitfall"] == pitfall)
        .unwrap_or_else(|| panic!("missing boundary for {pitfall}"))
}

fn array_contains(value: &Value, needle: &str) -> bool {
    value
        .as_array()
        .expect("expected JSON array")
        .iter()
        .any(|entry| entry.as_str() == Some(needle))
}

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn aggregate_tooling_does_not_claim_student_or_authority_use() {
    // Checks SLATE-PF-02.
    let readme = include_str!("../../../README.md");
    let product_plan = include_str!("../../../PRODUCT_PLAN.md");
    let adoption = include_str!("../../../docs/adoption/README.md");
    let review = include_str!("../../../docs/vtrace/REVIEW.md");
    let roles = include_str!("../../../.roles/ROLE.md");
    let manifest = public_claim_boundaries();
    let boundary = pitfall_boundary(&manifest, "SLATE-PF-02");

    assert!(readme.contains("not a pedagogical study"));
    assert!(readme.contains("student-level advice"));
    assert!(product_plan.contains("No student-level advice"));
    assert!(adoption.contains("aggregate-only seat"));
    assert!(review.contains("SLATE-PF-02"));
    assert!(roles.contains("Scope Keeper"));
    assert_eq!(boundary["required_owner"], "Scope Keeper");
    for blocked in [
        "student-level advice",
        "student record use",
        "assignment decision",
        "funding decision",
        "accreditation determination",
        "licensing determination",
        "institutional endorsement",
    ] {
        assert!(array_contains(&boundary["blocked_claims"], blocked));
    }
    for required in [
        "scope",
        "scale",
        "source_inventory",
        "demand_basis",
        "privacy_boundary",
        "governance_boundary",
        "role_review",
    ] {
        assert!(array_contains(&boundary["required_reuse_fields"], required));
    }
}

#[test]
fn fixture_validation_does_not_claim_public_corpus_validation() {
    // Checks SLATE-PF-05.
    let verification = include_str!("../../../docs/vtrace/VERIFICATION.md");
    let trace = include_str!("../../../docs/vtrace/TRACE.md");
    let interfaces = include_str!("../../../docs/vtrace/INTERFACES.md");
    let product_plan = include_str!("../../../PRODUCT_PLAN.md");
    let manifest = public_claim_boundaries();
    let boundary = pitfall_boundary(&manifest, "SLATE-PF-05");

    assert!(normalized(verification).contains("public aggregate source corpus work remains future"));
    assert!(verification.contains("pending public corpus"));
    assert!(trace.contains("does not prove a public aggregate education corpus"));
    assert!(interfaces.contains("public aggregate corpus compatibility remains"));
    assert!(product_plan.contains("Select a bounded public aggregate-data corpus"));
    assert_eq!(boundary["required_owner"], "Citation Auditor");
    for blocked in [
        "validated public education-system finding",
        "source-backed public corpus complete",
        "public adequacy result",
        "customer-ready corpus validation",
        "district readiness",
        "state readiness",
        "ministry readiness",
        "board readiness",
        "funder readiness",
    ] {
        assert!(array_contains(&boundary["blocked_claims"], blocked));
    }
    for gate in [
        "source review",
        "privacy review",
        "governance review",
        "scale declaration",
        "demand-basis declaration",
        "citation audit",
        "parliament review",
        "editorial review",
    ] {
        assert!(array_contains(
            &boundary["required_public_corpus_gates"],
            gate
        ));
    }
}
