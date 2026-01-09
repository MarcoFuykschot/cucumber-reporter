
all: build test docs

build:
    cargo build --release

test:
    cargo test --test main -- --output-html-path docs/

docs:
    cargo doc --no-deps --document-private-items