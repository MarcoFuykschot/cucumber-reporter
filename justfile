all: build test docs

build:
    cargo build --release

test:
    cargo test --test main -- --output-html-path docs/

docs:
    cargo doc --no-deps --document-private-items

test-template:
    cargo test --test main -- --output-html-path docs/ --template-dir tests/template_dark_mode_example/
