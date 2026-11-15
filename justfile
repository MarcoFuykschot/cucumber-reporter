all: build test check docs

build:
    cargo build --release

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu

build-musl:
    cargo build --release --target x86_64-unknown-linux-musl

test:
    cargo test --lib
    cargo test --test typst_summary replaces_underscores_in_table_cells
    cargo test --test typst_summary renders_background_and_rule_sections
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_without.pdf
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_withinfo.pdf --run-information features/reportinfo.yaml
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_with_typst_info.pdf --run-information features/reportinfo-typst.yaml
    
test-musl: build-musl
    target/x86_64-unknown-linux-musl/release/report-generator --spec-path "features/**/*.feature" --output-document docs/musl-report.pdf

install-tools:
    sudo apt install -y libssl-dev pkg-config musl-tools git-lfs
    rustup target add x86_64-unknown-linux-musl

check:
    cargo clippy --all-targets --all-features -- -D warnings

docs:
    cargo doc --no-deps --document-private-items

download-meta:
    curl -o schemas/gherkin-languages.json https://raw.githubusercontent.com/cucumber/gherkin/refs/heads/main/gherkin-languages.json
    curl -o schemas/canonical.json https://raw.githubusercontent.com/cucumber/cucumber-json-schema/refs/heads/main/schemas/canonical.json