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

#[test]
fn includes_feature_parse_errors_in_typst_output() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-parse-error-{}.html",
        std::process::id()
    ));
    let spec_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("features/feature-syntaxt-error.feature");
    let results_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-results-{}-{}.json",
        std::process::id(),
        std::thread::current().name().unwrap_or("main").len()
    ));

    std::fs::write(
        &results_path,
        r#"[
            {
              "uri": "features/feature-syntaxt-error.feature",
              "keyword": "",
              "name": "",
              "tags": [],
              "elements": [
                {
                  "keyword": "",
                  "type": "scenario",
                  "id": "failed-to-parsefeatures/feature-syntaxt-error.feature",
                  "line": 0,
                  "name": "",
                  "tags": [],
                  "steps": [
                    {
                      "keyword": "",
                      "line": 0,
                      "name": "scenario",
                      "result": {
                        "status": "failed",
                        "duration": 0,
                        "error_message": "Could not parse feature file: /tmp/feature-syntaxt-error.feature"
                      }
                    }
                  ]
                }
              ]
            }
        ]"#,
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &Some(vec![results_path.clone()]),
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Could not parse feature file"));
    assert!(html.contains("feature-syntaxt-error"));

    std::fs::remove_file(&results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn includes_multiple_feature_errors_in_typst_output() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-multi-error-{}.html",
        std::process::id()
    ));
    let spec_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("features/feature-simple.feature");
    let results_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-multi-error-results-{}-{}.json",
        std::process::id(),
        std::thread::current().name().unwrap_or("main").len()
    ));

    std::fs::write(
        &results_path,
        r#"[
            {
              "uri": "features/feature-simple.feature",
              "keyword": "Feature",
              "name": "simple",
              "tags": [],
              "elements": [
                {
                  "keyword": "Scenario",
                  "type": "scenario",
                  "id": "simple/scenario-1",
                  "line": 5,
                  "name": "Scenario 1",
                  "tags": [],
                  "steps": [
                    {
                      "keyword": "Given ",
                      "line": 6,
                      "name": "a fact",
                      "result": {
                        "status": "failed",
                        "duration": 0,
                        "error_message": "First error message"
                      }
                    },
                    {
                      "keyword": "When ",
                      "line": 7,
                      "name": "something is executed",
                      "result": {
                        "status": "failed",
                        "duration": 0,
                        "error_message": "Second error message"
                      }
                    }
                  ]
                }
              ]
            }
        ]"#,
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &Some(vec![results_path.clone()]),
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Errors"));
    assert!(html.contains("First error message"));
    assert!(html.contains("Second error message"));

    std::fs::remove_file(&results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn includes_outline_error_column_in_typst_output() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-outline-error-{}.html",
        std::process::id()
    ));
    let spec_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("features/feature-with-outline.feature");
    let results_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-outline-results-{}-{}.json",
        std::process::id(),
        std::thread::current().name().unwrap_or("main").len()
    ));

    std::fs::write(
        &results_path,
        r#"[
            {
              "uri": "features/feature-with-outline.feature",
              "keyword": "Feature",
              "name": "outline",
              "tags": [],
              "elements": [
                {
                  "keyword": "Scenario Outline",
                  "type": "scenario",
                  "id": "outline/scenario-outline-name-test",
                  "line": 3,
                  "name": "Scenario Outline name <test>",
                  "tags": [],
                  "steps": [
                    {
                      "keyword": "Given ",
                      "line": 5,
                      "name": "a fact with '<Header1>'",
                      "result": {
                        "status": "failed",
                        "duration": 0,
                        "error_message": "Example row failed: Header1=Value 2"
                      }
                    }
                  ]
                }
              ]
            }
        ]"#,
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &Some(vec![results_path.clone()]),
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Error"));
    assert!(html.contains("Example row failed: Header1=Value 2"));

    std::fs::remove_file(&results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}
