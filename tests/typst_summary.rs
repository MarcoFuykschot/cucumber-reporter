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
    assert!(html.contains("<td>Scenario</td><td>Result</td>"));
    assert!(!html.contains("<td>Rule</td><td>Scenario</td>"));
    assert!(html.contains("<td><a href=\"#feature-0-scenario-5\">Scenario 1</a></td>"));
    assert!(html.contains("<h2 id=\"feature-0\">1. simple</h2>"));
    assert!(html.contains("<h3 id=\"feature-0-scenario-5\">1.3. Scenario 1</h3>"));

    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn hides_rule_cell_for_standalone_scenarios_in_mixed_summary() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-html-mixed-summary-{}.html",
        std::process::id()
    ));
    let spec_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-mixed-summary-{}.feature",
        std::process::id()
    ));
    std::fs::write(
        &spec_path,
        "Feature: mixed scenarios\n\
         Scenario: Standalone scenario\n\
           Given a fact\n\
         Rule: A rule\n\
           Scenario: Nested scenario\n\
             Given a fact\n",
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("<td>Rule</td><td>Scenario</td><td>Result</td>"));
    assert!(html.contains(
        "<td><a href=\"#feature-0-scenario-2\">Standalone scenario</a></td><td></td>"
    ));
    assert!(html.contains("<td colspan=\"2\">A rule</td>"));
    assert!(html.contains(
        "<td></td><td><a href=\"#feature-0-scenario-5\">Nested scenario</a></td>"
    ));
    assert!(html.contains("Scenario summary"));

    std::fs::remove_file(spec_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn includes_feature_summary_and_rules_list_in_html() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-html-summary-{}.html",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-with-rules.feature");

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Rules and scenarios summary"));
    assert!(!html.contains("Scenario summary"));
    assert!(html.contains("<td>Result</td>"));
    assert!(html.contains("rule 1</td>"));
    assert!(html.contains("<td>Rule</td><td>Scenario</td><td>Result</td>"));
    assert!(html.contains("<td colspan=\"2\">rule 1</td>"));
    assert!(html.contains("<td colspan=\"2\">rule 2</td>"));
    assert!(html.contains("<a href=\"#feature-0-scenario-6\">Scenario 1</a>"));
    assert!(html.contains("<a href=\"#feature-0-scenario-34\">Scenario 1</a>"));
    assert!(!html.contains("<td>Number</td>"));
    assert!(!html.contains("<td>1</td><td>Scenario 1</td>"));
    assert!(html.contains("Scenario description"));
    assert!(html.contains("Description test scenario"));
    assert!(html.contains("Description of a rule"));

    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn matches_rule_prefixed_results_without_error_details_in_summary() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-rule-outcomes-{}.html",
        std::process::id()
    ));
    let results_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-rule-outcomes-{}.json",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-with-rules.feature");

    std::fs::write(
        &results_path,
        r#"[
            {
              "uri": "features/feature-with-rules.feature",
              "keyword": "Feature",
              "name": "rules",
              "tags": [],
              "elements": [
                {
                  "keyword": "Scenario",
                  "type": "scenario",
                  "id": "scenario-1",
                  "line": 6,
                  "name": "rule 1 Scenario 1",
                  "tags": [],
                  "steps": [
                    {
                      "keyword": "Given",
                      "line": 7,
                      "name": "a fact",
                      "result": {
                        "status": "failed",
                        "duration": 0,
                        "error_message": "First rule failure"
                      }
                    }
                  ]
                },
                {
                  "keyword": "Scenario",
                  "type": "scenario",
                  "id": "scenario-2",
                  "line": 34,
                  "name": "rule 2 Scenario 1",
                  "tags": [],
                  "steps": [
                    {
                      "keyword": "Given",
                      "line": 35,
                      "name": "a fact",
                      "result": {
                        "status": "skipped",
                        "duration": 0,
                        "error_message": "Second rule skipped"
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
    assert!(html.contains("<td colspan=\"2\">rule 1</td><td>failed</td>"));
    assert!(html.contains("<td colspan=\"2\">rule 2</td><td>skipped</td>"));
    let summary = html
        .split("Rules and scenarios summary</h3>")
        .nth(1)
        .unwrap()
        .split("</table>")
        .next()
        .unwrap();
    assert!(summary.contains("<td colspan=\"2\">rule 1</td><td>failed</td>"));
    assert!(summary.contains(
        "<td><a href=\"#feature-0-scenario-6\">Scenario 1</a></td><td>failed</td>"
    ));
    assert!(summary.contains("<td colspan=\"2\">rule 2</td><td>skipped</td>"));
    assert!(summary.contains(
        "<td><a href=\"#feature-0-scenario-34\">Scenario 1</a></td><td>skipped</td>"
    ));
    assert_eq!(summary.matches("<tr>").count(), 6);
    assert!(!summary.contains("First rule failure"));
    assert!(!summary.contains("Second rule skipped"));

    std::fs::remove_file(results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn groups_outline_results_without_error_details_in_summary() {
    let base_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-outline-summary-{}",
        std::process::id()
    ));
    let spec_path = base_path.with_extension("feature");
    let results_path = base_path.with_extension("json");
    let output_path = base_path.with_extension("html");
    std::fs::write(
        &spec_path,
        "Feature: Outline summary\n\
         Scenario: Shared name\n\
           Given a fact\n\
         Scenario Outline: Standalone <value>\n\
           Given value <value>\n\
           Examples:\n\
             | value |\n\
             | <other> |\n\
         Rule: A rule\n\
           Scenario Outline: Shared <value>\n\
             Given value <value>\n\
             Examples:\n\
               | value |\n\
               | first |\n\
               | second |\n",
    )
    .unwrap();
    let elements = [
        (2, "Shared name", "Standalone failure"),
        (14, "A rule Shared name", "First example failure"),
        (15, "A rule Shared name", "Second example failure"),
        (0, "Shared name", "Unmatched failure"),
    ]
    .map(|(line, name, message)| {
        serde_json::json!({
            "keyword": "Scenario",
            "type": "scenario",
            "id": format!("scenario-{line}"),
            "line": line,
            "name": name,
            "tags": [],
            "future_element_field": true,
            "steps": [{
                "keyword": "Given",
                "line": line,
                "name": "a fact",
                "future_step_field": "ignored",
                "result": {
                    "status": "failed",
                    "duration": 0,
                    "error_message": message,
                    "future_result_field": null
                }
            }]
        })
    });
    let results = serde_json::json!([{
        "uri": spec_path.file_name().unwrap().to_str().unwrap(),
        "keyword": "Feature",
        "name": "Outline summary",
        "future_feature_field": {"ignored": true},
        "tags": [],
        "elements": elements
    }]);
    std::fs::write(&results_path, results.to_string()).unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &Some(vec![results_path.clone()]),
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    let contents = html.split("</nav>").next().unwrap();
    assert!(contents.contains("Shared value"));
    assert!(!contents.contains("&lt;value>"));
    let rule_summary = html
        .split("Rules and scenarios summary</h3>")
        .nth(1)
        .unwrap()
        .split("</table>")
        .next()
        .unwrap();
    assert!(rule_summary.contains("<td colspan=\"2\">A rule</td><td>failed</td>"));
    assert!(rule_summary.contains(
        "<td></td><td><a href=\"#feature-0-scenario-10\">Shared value</a></td><td>failed</td>"
    ));
    let scenario_summary = html
        .split("Scenario summary</h3>")
        .nth(1)
        .unwrap()
        .split("</table>")
        .next()
        .unwrap();
    assert!(scenario_summary.contains(
        "<td><a href=\"#feature-0-scenario-4\">Standalone value</a></td><td>passed</td>"
    ));
    assert!(!scenario_summary.contains("&lt;value&gt;"));
    assert!(!html.contains("&lt;value>"));
    assert!(html.contains("<td>other</td>"));
    assert!(
        scenario_summary
            .contains("<td><a href=\"#feature-0-scenario-2\">Shared name</a></td><td>failed</td>")
    );
    assert_eq!(scenario_summary.matches("<tr>").count(), 4);
    assert!(!scenario_summary.contains("Standalone failure"));
    assert!(!scenario_summary.contains("First example failure"));
    assert!(!scenario_summary.contains("Second example failure"));
    assert!(!scenario_summary.contains("Unmatched failure"));

    std::fs::remove_file(spec_path).unwrap();
    std::fs::remove_file(results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn renders_feature_introduction_from_markdown_file() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-{}.html",
        std::process::id()
    ));
    let introduction_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-{}.md",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    std::fs::write(&introduction_path, "**Markdown introduction**").unwrap();
    let run_info: ReportInfo = serde_yaml::from_str(&format!(
        "feature_introduction: '{}'",
        introduction_path.display()
    ))
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Markdown introduction"));
    assert!(!html.contains("**"));

    std::fs::remove_file(introduction_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn customizes_section_headings_from_report_info_yaml() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-custom-headings-{}.html",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-with-rules.feature");
    let run_info: ReportInfo = serde_yaml::from_str(
        r#"headings:
  features: "Feature overview"
  rules_and_scenarios_summary: "Rule summary"
  requirements: "Rule details"
  tag_index: "Tags""#,
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    let (contents, body) = html.split_once("</nav>").unwrap();
    assert!(!contents.contains("Feature overview"));
    assert!(body.contains("font-weight: bold; margin-block: 0.83em\">Feature overview</div>"));
    assert!(body.contains("<h2 id=\"feature-0\">1. rules</h2>"));
    assert!(body.contains("<h4 id=\"feature-0-scenario-6\">1.3.1. Scenario 1</h4>"));
    assert!(html.contains("Rule summary"));
    assert!(html.contains("Rule details"));
    assert!(html.contains("Tags"));
    assert!(!html.contains("Features"));
    assert!(!html.contains("Rules and scenarios summary"));
    assert!(!html.contains("Requirements"));
    assert!(!html.contains("Tag index"));

    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn customizes_summary_and_step_table_headers_from_report_info_yaml() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-custom-table-headers-{}.html",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-with-rules.feature");
    let run_info: ReportInfo = serde_yaml::from_str(
        r#"table_headers:
  feature: "Capability"
  rule: "Requirement"
  scenario: "Example"
  result: "Status"
  step: "Action""#,
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("<td>Capability</td><td>Status</td>"));
    assert!(html.contains("<td>Requirement</td><td>Example</td><td>Status</td>"));
    assert!(html.contains("<td colspan=\"2\">Action</td><td>Status</td>"));
    assert!(!html.contains("<td>Feature</td><td>Result</td>"));
    assert!(!html.contains("<td>Rule</td><td>Scenario</td><td>Result</td>"));
    assert!(!html.contains("<td colspan=\"2\">Step</td><td>Result</td>"));

    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn renders_header_and_footer_from_markdown_files() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-header-footer-{}.html",
        std::process::id()
    ));
    let header_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-header-{}.md",
        std::process::id()
    ));
    let footer_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-footer-{}.md",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    std::fs::write(&header_path, "**Markdown header**").unwrap();
    std::fs::write(&footer_path, "*Markdown footer*").unwrap();
    let run_info: ReportInfo = serde_yaml::from_str(&format!(
        "header: '{}'\nfooter: '{}'",
        header_path.display(),
        footer_path.display()
    ))
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Markdown header"));
    assert!(html.contains("Markdown footer"));
    assert!(!html.contains("**Markdown header**"));
    assert!(!html.contains("*Markdown footer*"));
    let header_position = html.find("Markdown header").unwrap();
    let contents_end = html.find("</nav>").unwrap();
    let features_position = html.find(">Features</div>").unwrap();
    assert!(contents_end < header_position);
    assert!(header_position < features_position);

    std::fs::remove_file(header_path).unwrap();
    std::fs::remove_file(footer_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn renders_custom_title_page_from_markdown_file() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-title-page-{}.html",
        std::process::id()
    ));
    let title_page_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-title-page-{}.md",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    std::fs::write(&title_page_path, "# Custom title page\n\n**Custom author**").unwrap();
    let run_info: ReportInfo = serde_yaml::from_str(&format!(
        "title: 'Default report title'\nauthor: 'Default report author'\ntitle_page: '{}'",
        title_page_path.display()
    ))
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("Custom title page"));
    assert!(html.contains("Custom author"));
    assert!(!html.contains("<h1>Default report title</h1>"));
    assert!(!html.contains("<p>Default report author</p>"));

    std::fs::remove_file(title_page_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn renders_typst_templates_from_reportinfo_yaml() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-typst-templates-{}.html",
        std::process::id()
    ));
    let pdf_output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-typst-templates-{}.pdf",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    let report_info_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/reportinfo-typst.yaml");
    let run_info: ReportInfo =
        serde_yaml::from_str(&std::fs::read_to_string(report_info_path).unwrap()).unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("<strong>Typst title page example</strong>"));
    assert!(html.contains("<strong>Typst header example</strong>"));
    assert!(html.contains("<strong>Typst footer example</strong>"));
    let history = html
        .split("Document history</h2>")
        .nth(1)
        .unwrap()
        .split("</table>")
        .next()
        .unwrap();
    assert!(history.contains("<strong>Author</strong>"));
    assert!(history.contains("<strong>Version</strong>"));
    assert!(history.contains("<strong>Changes</strong>"));
    assert!(history.contains("Initial draft"));
    assert!(history.contains("Reviewed and approved for publication"));

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        pdf_output_path.to_str().unwrap(),
        ReportFormat::Pdf,
        &run_info,
    )
    .unwrap();
    assert!(std::fs::metadata(&pdf_output_path).unwrap().len() > 0);

    std::fs::remove_file(output_path).unwrap();
    std::fs::remove_file(pdf_output_path).unwrap();
}

#[test]
fn renders_feature_introduction_from_typst_file() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-typst-{}.html",
        std::process::id()
    ));
    let introduction_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-{}.typ",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    std::fs::write(&introduction_path, "#strong[Typst introduction]").unwrap();
    let run_info: ReportInfo = serde_yaml::from_str(&format!(
        "feature_introduction: '{}'",
        introduction_path.display()
    ))
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("<strong>Typst introduction</strong>"));

    std::fs::remove_file(introduction_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn renders_feature_introduction_from_txt_file_as_plain_text() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-plain-{}.html",
        std::process::id()
    ));
    let introduction_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-{}.txt",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    std::fs::write(&introduction_path, "**Plain-text file introduction**").unwrap();
    let run_info: ReportInfo = serde_yaml::from_str(&format!(
        "feature_introduction: '{}'",
        introduction_path.display()
    ))
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(html.contains("**Plain-text file introduction**"));

    std::fs::remove_file(introduction_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn rejects_feature_introduction_that_is_not_a_file() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-introduction-string-{}.html",
        std::process::id()
    ));
    let spec_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features/feature-simple.feature");
    let run_info: ReportInfo =
        serde_yaml::from_str("feature_introduction: 'Literal **introduction**'").unwrap();

    let result = ReportGenerator::generate_report(
        &vec![spec_path],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &run_info,
    );

    assert!(result.unwrap_err().to_string().contains("existing .md, .typ, or .txt file"));
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
    assert!(!html.contains("<td>Errors</td>"));
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
    assert!(!html.contains("<td>Errors</td>"));
    assert!(html.contains("First error message"));
    assert!(html.contains("Second error message"));
    let summary = html
        .split("Scenario summary</h3>")
        .nth(1)
        .unwrap()
        .split("</table>")
        .next()
        .unwrap();
    assert_eq!(
        summary
            .matches("<a href=\"#feature-0-scenario-5\">Scenario 1</a>")
            .count(),
        1
    );
    assert!(!summary.contains("First error message"));
    assert!(!summary.contains("Second error message"));

    std::fs::remove_file(&results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn omits_step_outcome_column_for_outline_in_typst_output() {
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
    assert!(html.contains("<td>Keyword</td><td>Step</td>"));
    assert!(!html.contains("<td>Keyword</td><td>Step</td><td>Outcome</td>"));

    std::fs::remove_file(&results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn replaces_underscores_in_table_cells() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-table-cells-{}.pdf",
        std::process::id()
    ));
    let html_output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-table-cells-{}.html",
        std::process::id()
    ));
    let spec_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-table-cells-{}.feature",
        std::process::id()
    ));
    std::fs::write(
        &spec_path,
        "Feature: Wide table headers\n\
         Scenario: Step table\n\
         Given values\n\
         | Header1 | Header2 | test | a long column name | col2 | a bit longer column name | very long column name | an other very long column name | A other exta long column name to test formating | a_very_long_unbreakable_column_name_to_force_wrapping |\n\
         | Value 1 | Value 2 | 3 | | | | | | | a_value_without_spaces |\n\
         Scenario Outline: Examples table\n\
         Given a value\n\
         Examples:\n\
         | Header1 | Header2 | test | a long column name | col2 | a bit longer column name | very long column name | an other very long column name | A other exta long column name to test formating | a_very_long_unbreakable_column_name_to_force_wrapping |\n\
         | Value 1 | Value 2 | 3 | | | | | | | |\n",
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Pdf,
        &ReportInfo::default(),
    )
    .unwrap();

    let pdf = std::fs::read(&output_path).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &None,
        html_output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&html_output_path).unwrap();
    assert!(html.matches("A other exta long column name to test formating").count() >= 2);
    assert!(
        html.matches("a very long unbreakable column name to force wrapping")
            .count()
            >= 2
    );
    assert!(html.contains("a value without spaces"));

    std::fs::remove_file(spec_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
    std::fs::remove_file(html_output_path).unwrap();
}

#[test]
fn renders_background_and_rule_sections() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-background-{}.pdf",
        std::process::id()
    ));
    let spec_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-background-{}.feature",
        std::process::id()
    ));
    std::fs::write(
        &spec_path,
        "Feature: Background styling\n\
         Background:\n\
         Given a shared state\n\
         Scenario: Background styling\n\
         When an action is performed\n\
         Then a result is shown\n\
         @RuleTag\n\
         Rule: Rule styling\n\
         A rule description\n\
         Scenario: Rule scenario\n\
         Given a rule-specific state\n",
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Pdf,
        &ReportInfo::default(),
    )
    .unwrap();

    let pdf = std::fs::read(&output_path).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));

    std::fs::remove_file(spec_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn renders_step_error_on_next_line_in_step_cell() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-step-error-{}.html",
        std::process::id()
    ));
    let spec_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-step-error-{}.feature",
        std::process::id()
    ));
    let results_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-step-error-{}.json",
        std::process::id()
    ));
    std::fs::write(
        &spec_path,
        "Feature: Step errors\n\
         Scenario: Failing step\n\
         Given a value\n",
    )
    .unwrap();

    let results = serde_json::json!([{
        "uri": spec_path.file_name().unwrap().to_string_lossy(),
        "keyword": "Feature",
        "name": "Step errors",
        "tags": [],
        "elements": [{
            "keyword": "Scenario",
            "type": "scenario",
            "line": 2,
            "name": "Failing step",
            "tags": [],
            "steps": [{
                "keyword": "Given ",
                "line": 3,
                "name": "a value",
                "result": {
                    "status": "failed",
                    "duration": 0,
                    "error_message": "step error marker"
                }
            }]
        }]
    }]);
    std::fs::write(&results_path, results.to_string()).unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &Some(vec![results_path.clone()]),
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    let step_position = html.find("a value").unwrap();
    let error_position = step_position + html[step_position..]
        .find("Error: step error marker")
        .unwrap();
    let between = &html[step_position..error_position];
    assert!(between.contains("<br"));
    assert!(!between.contains("</td>"));

    std::fs::remove_file(spec_path).unwrap();
    std::fs::remove_file(results_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn omits_empty_background_section() {
    let output_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-empty-background-{}.html",
        std::process::id()
    ));
    let spec_path = std::env::temp_dir().join(format!(
        "cucumber-reporter-empty-background-{}.feature",
        std::process::id()
    ));
    std::fs::write(
        &spec_path,
        "Feature: Empty background\n\
         Background:\n\
         Scenario: Has its own step\n\
         Given a value\n",
    )
    .unwrap();

    ReportGenerator::generate_report(
        &vec![spec_path.clone()],
        &None,
        output_path.to_str().unwrap(),
        ReportFormat::Html,
        &ReportInfo::default(),
    )
    .unwrap();

    let html = std::fs::read_to_string(&output_path).unwrap();
    assert!(!html.contains("Background"));

    std::fs::remove_file(spec_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}
