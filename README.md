## Goal

This project generates Typst reports from Gherkin feature files and optional Cucumber JSON execution results. Reports can be exported as PDF or experimental Typst HTML.

The standalone report generator accepts a glob for feature files, optionally matches those files against JSON output from your test run, and combines the result with report metadata from a YAML file. Reports are generated after the test run; there is no live Cucumber writer integration.

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
- `--report-format <pdf|html>`: output format. `pdf` is the default; `html` is an experimental export directly from Typst and may differ from the PDF output.
- `--output-document <PATH>`: destination file for the generated report.

### Run metadata example

```yaml
title: "A report on features"
author: "By someone"
sub_title: "Subtile is a nice title"
time_run: "2026-10-15T15:15:15Z"
feature_introduction: "features/feature-introduction.md"
title_page: "features/report-title-page.md"
header: "features/report-header.md"
footer: "features/report-footer.md"
```

`feature_introduction` must be the path to an existing `.md`, `.typ`, or `.txt`
file. Markdown files are rendered as Markdown, Typst files as Typst markup, and
text files as plain text.

The optional `title_page` value must point to an existing `.md` or `.typ` file.
Its content replaces the default title-and-author page and can span multiple
pages. Markdown files are rendered as Markdown and Typst files as Typst markup.
When omitted, the default title page is used.

The optional `header` and `footer` values must point to existing `.md` or `.typ`
files. Their content is rendered in the header or footer on each PDF page,
except for the title page. In experimental HTML output, the header appears
after the contents list and the footer at the end of the document. Markdown
files are rendered as Markdown and Typst files as Typst markup.

A complete example using Typst files for the title page, header, and footer is
available in [features/reportinfo-typst.yaml](features/reportinfo-typst.yaml).
It includes a centered cover followed by a second page with an author,
version, date, and change-history table.

### Example: PDF report with Cucumber JSON results

```sh
cargo run --bin report-generator -- \
  --spec-path "features/**/*.feature" \
  --json-results-path "target/**/*.json" \
  --run-information "features/reportinfo.yaml" \
  --output-document "reports/feature-report.pdf"
```

If no JSON result files are supplied, the generator still renders a report from the feature definitions alone.

The scenario summary matches execution outcomes and errors by source line, so
rule-prefixed JSON scenario names are supported. Errors appear in the matching
scenario row; only errors without a matching scenario get separate rows.

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

## Experimental Typst HTML export
The standalone `report-generator` also supports experimental Typst HTML output. Typst's HTML export is incomplete, so the output may differ from the PDF report.

```sh
cargo run --bin report-generator -- --spec-path "features/**/*.feature" --report-format html --output-document report.html
```

## References

#### Cucumber
* [Cucumber-rs book](https://cucumber-rs.github.io/cucumber/current/)
* [Typst]
