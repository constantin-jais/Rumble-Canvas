use rumble_canvas_domain::sample_workspace;
use rumble_canvas_handoff::wrench_integration::{check_package_completeness, summarize_evidence};
use rumble_canvas_package::build_package;

#[test]
#[ignore] // Only runs if wrench-inspect is installed
fn test_wrench_check_passes_on_sample_package() {
    let workspace = sample_workspace();
    let package = build_package(&workspace).expect("sample package builds");

    let evidence = check_package_completeness(&package).expect("wrench check runs");

    let (passed, _messages) = summarize_evidence(&evidence);
    assert!(passed, "Sample package passes wrench checks");
}
