use rumble_canvas_domain::sample_workspace;
use rumble_canvas_handoff::wrench_integration::{check_package_completeness, summarize_report};
use rumble_canvas_package::build_package;

#[test]
#[ignore] // Requires wrench-inspect on PATH; CI installs it and runs with --ignored.
fn test_wrench_check_passes_on_sample_package() {
    let workspace = sample_workspace();
    let package = build_package(&workspace).expect("sample package builds");

    let report = check_package_completeness(&workspace, &package).expect("wrench inspection runs");

    let (passed, messages) = summarize_report(&report);
    assert!(
        passed,
        "Sample package passes wrench checks; findings: {messages:?}"
    );
    assert_eq!(report.summary.errors, 0, "no error findings expected");
}
