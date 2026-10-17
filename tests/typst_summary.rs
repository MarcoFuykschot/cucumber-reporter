use std::path::PathBuf;

use cucumber_reporter::{ReportFormat, ReportGenerator, ReportInfo};

#[test]
fn exports_typst_as_html() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-html-{}.html",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains("Scenario 1"));

    std::fs::remove_file(output_path).unwrap();
}
