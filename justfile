all: build test check docs

build:
    cargo build --release

test:
    cargo test --test main -- --output-html-path docs/
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --json-results-path target/results.json --output-document docs/report_with_result.pdf
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_without.pdf
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_withinfo.pdf --run-information features/reportinfo.yaml
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --json-results-path target/results.json --output-document docs/report_with_result.html --report-format html

install-tools:
    echo "no tools"

check:
    cargo clippy --all-targets --all-features -- -D warnings

docs:
    cargo doc --no-deps --document-private-items

test-template:
    cargo test --test main -- --output-html-path docs/ --template-dir tests/template_dark_mode_example/

download-meta:
    curl -o schemas/gherkin-languages.json https://raw.githubusercontent.com/cucumber/gherkin/refs/heads/main/gherkin-languages.json
    curl -o schemas/canonical.json https://raw.githubusercontent.com/cucumber/cucumber-json-schema/refs/heads/main/schemas/canonical.json