## Goal

This project generates a report from Gherkin feature files and optional Cucumber JSON execution results. The report can be exported as a PDF or, experimentally, as HTML.

The standalone report generator accepts a glob for feature files, optionally matches those files against JSON output from your test run, and combines the result with report metadata from a YAML file before rendering the final document.

## Standalone report generator

The CLI is available as the `report-generator` binary:

```sh
cargo run --bin report-generator -- \
  --spec-path "features/**/*.feature" \
  [--json-results-path "target/**/*.json"] \
  [--run-information "features/reportinfo.yaml"] \
  [--report-format pdf|html] \
  --output-document "report.pdf"
```

### Options

- `--spec-path <GLOB>`: required glob for the feature files to include in the report.
- `--json-results-path <GLOB>`: optional glob for Cucumber JSON result files. When present, each JSON feature is matched back to its corresponding feature file and merged into the final report.
- `--run-information <PATH>`: optional YAML file containing run metadata such as title, author, subtitle and timestamp.
- `--report-format <pdf|html>`: output format. `pdf` is the default and stable option. `html` is experimental and may differ from the PDF output.
- `--output-document <PATH>`: destination file for the generated report.

### Run metadata example

```yaml
title: "A report on features"
author: "By someone"
sub_title: "Subtile is a nice title"
time_run: "2026-10-15T15:15:15Z"
```

### Example: PDF report with Cucumber JSON results

```sh
cargo run --bin report-generator -- \
  --spec-path "features/**/*.feature" \
  --json-results-path "target/**/*.json" \
  --run-information "features/reportinfo.yaml" \
  --output-document "reports/feature-report.pdf"
```

### Example: HTML report without execution results

```sh
cargo run --bin report-generator -- \
  --spec-path "features/**/*.feature" \
  --report-format html \
  --output-document "report.html"
```

If no JSON result files are supplied, the generator still renders a report from the feature definitions alone.

## Two ways to generate a report

This project supports two complementary ways to produce output:

1. The standalone `report-generator` binary, which reads feature files and optional Cucumber JSON results and writes a final PDF or HTML report to disk. This is the easiest option for a single report generation step in CI, scripts, or local runs.
2. The Rust library integration, where `CucumberReporter` is attached to an existing Cucumber test run. This is useful when your test suite already produces regular Cucumber output and you want the report HTML generated as part of that run without creating a separate command.

### 1. Standalone generator (CLI)

This is the method shown above: point the tool at a feature glob and optionally a JSON result glob, then write the final document to a file.

```sh
cargo run --bin report-generator -- \
  --spec-path "features/**/*.feature" \
  --json-results-path "target/**/*.json" \
  --run-information "features/reportinfo.yaml" \
  --output-document "reports/feature-report.pdf"
```

This mode is best when you want a single generated report artifact from a known set of feature files.

### 2. Cucumber integration (Rust library)

If you already run Cucumber tests from Rust, you can tee `CucumberReporter` into the writer pipeline. The reporter collects the same feature data while the normal Cucumber output continues to flow to stdout and/or JSON.

```rust
use cucumber::{
    World, WriterExt,
    writer::{self, Basic},
};
use cucumber_reporter::CucumberReporter;

#[derive(World, Debug, Default)]
struct ReporterWorld;

#[tokio::main]
async fn main() {
    ReporterWorld::cucumber()
        .with_default_cli()
        .with_writer(
            Basic::stdout()
                .summarized()
                .tee::<ReporterWorld, _>(writer::Json::for_tee(
                    std::fs::File::create("target/results.json").unwrap(),
                ))
                .tee::<ReporterWorld, _>(CucumberReporter::new())
                .normalized(),
        )
        .run("features")
        .await;
}
```

This mode is best when the report should be produced as part of the existing test execution flow, instead of as a separate post-processing step.

## Examples

### A simple gherkin feature file

<!--CONTENT-START:features/feature-simple.feature:Feature-->
```Feature
# Some comments
Feature: simple
    With a description

    Scenario: Scenario 1
        Given a fact
        When something is executed
        Then the result is oke

    Scenario: Scenario 2
        Given a fact
        And a other fact
    # Some comments
        When something is executed
        """
        *Then the result is oke* also some markdown
        """
        Then the result is failed

    # Some comments

    @Tag1
    Scenario: Scenario 3 with markdown

        test header 1
        ==

        test header 2
        ---

        markdown test
        - test 1
        - test 2
        <br>
        *Bold text*
        <br>
        ~emphasis text~
        #  will be rendered as header 1
        ## will be rendered as header 2
        ```rust
        fn main() {
            println!("Hello, world!");
        }
        ```

        Given a fact
        Then a Skipped line
```
<!--CONTENT-END:features/feature-simple.feature-->

Will produce the following [html output](https://marcofuykschot.github.io/cucumber-reporter/F13421587706030853887.html)

### A gherkin feature using an outline

<!--CONTENT-START:features/feature-with-outline.feature:Feature-->
```Feature
Feature: outline

    Scenario Outline: Scenario Outline name <test>

        Given a fact with '<Header1>'

        Given a fact with '<Header2>'

        Examples:
            | Header1 | Header2 | test |
            | Value 1 | Value 1 | 1    |
            | Value 2 | Value 2 | 2    |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |

    Scenario Outline: Scenario with skipped
        Given a skipped fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |


    Scenario Outline: Scenario with skipped extra new line
        Given a skipped fact with '<Header1>'

        Examples:

            | Header1 | Header2 | test |

            | Value 3 | Value 3 | 3 |

            | Value 5 | Value 4 | 4 |

```
<!--CONTENT-END:features/feature-with-outline.feature-->

Will produce the following [html output](https://marcofuykschot.github.io/cucumber-reporter/F14974020654905966167.html)

## Templating
Currenlty templating functions by using html with basic css.
For templating, the [handlebars](https://docs.rs/handlebars/latest/handlebars/) rust crate is used.
For an example to use it. Refer to the default template in the `templates/` folder.

## Experimental Typst HTML export
The standalone `report-generator` also supports experimental Typst HTML output. Typst's HTML export is incomplete, so the output may differ from the PDF report.

```sh
cargo run --bin report-generator -- --spec-path "features/**/*.feature" --report-format html --output-document report.html
```

## References

#### Cucumber
* [Cucumber-rs book](https://cucumber-rs.github.io/cucumber/current/)
#### Templating
* [Handlebars](https://docs.rs/handlebars/latest/handlebars/) - crate used.
* [Handlebarsjs](https://handlebarsjs.com/guide/) - better guide for templating.
* [Typst]

