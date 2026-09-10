all: build test check docs

build:
    cargo build --release

test:
    cargo test --test main -- --output-html-path docs/
    cargo run --bin report-generator -- --spec-base-path . --json-results-path target/results.json --output-path docs/

install-tools:
    echo "no tools"

check:
    cargo clippy --all-targets --all-features -- -D warnings

docs:
    cargo doc --no-deps --document-private-items

test-template:
    cargo test --test main -- --output-html-path docs/ --template-dir tests/template_dark_mode_example/
