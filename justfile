all: build test check docs

build:
    cargo build --release

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu

build-musl:
    cargo build --release --target x86_64-unknown-linux-musl

test:
    cargo test --test main 
    cargo test --lib
    cargo test --test typst_summary replaces_underscores_in_table_cells
    cargo test --test typst_summary renders_background_and_rule_sections
    mkdir -p docs
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_without.pdf
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_withinfo.pdf --run-information features/reportinfo.yaml
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --output-document docs/report_with_typst_info.pdf --run-information features/reportinfo-typst.yaml
    cargo run --bin report-generator -- --spec-path "features/**/*.feature" --json-results-path target/results.json --output-document docs/report_with_result.pdf
    
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
    #!/usr/bin/env bash
    set -euo pipefail

    languages_tmp=''
    canonical_tmp=''
    cleanup() {
        [[ -z "$languages_tmp" ]] || rm -f -- "$languages_tmp"
        [[ -z "$canonical_tmp" ]] || rm -f -- "$canonical_tmp"
    }
    trap cleanup EXIT

    languages_tmp=$(mktemp schemas/.gherkin-languages.json.XXXXXX)
    canonical_tmp=$(mktemp schemas/.canonical.json.XXXXXX)

    curl --fail --location --silent --show-error \
        --output "$languages_tmp" \
        https://raw.githubusercontent.com/cucumber/gherkin/refs/heads/main/gherkin-languages.json \
        || { echo "Failed to download gherkin-languages.json" >&2; exit 1; }

    curl --fail --location --silent --show-error \
        --output "$canonical_tmp" \
        https://raw.githubusercontent.com/cucumber/cucumber-json-schema/refs/heads/main/schemas/canonical.json \
        || { echo "Failed to download canonical.json" >&2; exit 1; }

    patch --batch --forward --fuzz=0 --strip=1 "$canonical_tmp" < schemas/canonical.patch \
        || { echo "Failed to apply schemas/canonical.patch to canonical.json" >&2; exit 1; }

    mv "$languages_tmp" schemas/gherkin-languages.json
    mv "$canonical_tmp" schemas/canonical.json
