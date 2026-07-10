use rumble_canvas_domain::sample_workspace;
use rumble_canvas_package::{build_package, schema::validate_package};

#[test]
fn test_sample_package_passes_schema_validation() {
    let workspace = sample_workspace();
    let package = build_package(&workspace).expect("package builds");

    validate_package(&package).expect("sample package passes schema validation");
}

#[test]
fn test_package_schema_requires_metadata() {
    let workspace = sample_workspace();
    let mut package = build_package(&workspace).expect("package builds");

    // Clear metadata.created_by to trigger validation error
    package.metadata.created_by.clear();

    let result = validate_package(&package);
    assert!(
        result.is_err(),
        "Schema validation fails when metadata.created_by is missing"
    );
}

#[test]
fn test_package_schema_requires_delivery_target() {
    let workspace = sample_workspace();
    let mut package = build_package(&workspace).expect("package builds");

    package.delivery_target.handoff_format = "wrong-format".to_string();

    let result = validate_package(&package);
    assert!(
        result.is_err(),
        "Schema validation fails when handoff_format is incorrect"
    );
}

#[test]
fn test_package_schema_requires_valid_traceability_density() {
    let workspace = sample_workspace();
    let mut package = build_package(&workspace).expect("package builds");

    package.package_readiness_details.traceability_density = 1.5;

    let result = validate_package(&package);
    assert!(
        result.is_err(),
        "Schema validation fails when traceability_density is > 1.0"
    );
}
