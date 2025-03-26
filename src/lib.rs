#![doc = include_str!("../README.md")]

mod render_types;
mod reporter;
pub use reporter::CucumberReporter;


/// ```rust
/// use cucumber_reporter::docbuild::*;
/// copy_images();
/// ```
#[doc(hidden)]
pub mod docbuild {
    use std::{error::Error, fs, path::Path};

pub fn copy_images() -> Result<(),Box<dyn Error>>{
    let source = std::env::var("CARGO_SOURCE_DIR").unwrap_or_default();
    let assets_path = if std::env::var("DOCS_RS").is_ok() {
        Path::new(source.as_str())
    } else {
        Path::new(".")
    };

    let assets_path = assets_path.join("assets");

    let assets = std::fs::read_dir(assets_path)?;

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_default();
    let target = std::env::var("TARGET").unwrap_or_default();
    let doc_path = if std::env::var("DOCS_RS").is_ok() {
        Path::new(target_dir.as_str()).join(target.as_str())
    } else {
        Path::new("target").to_path_buf()
    };
    let doc_path = doc_path
        .join("doc")
        .join("cucumber_reporter")
        .join("assets");

    fs::create_dir_all(doc_path.clone())?;

    for asset in assets {
        let asset = asset?;
        let target_file = doc_path.join(asset.file_name());
        println!("cargo::warning={:?} {:?}", asset, target_file);
        std::fs::copy(asset.path(), target_file)?;
    }
     Ok(())
}
}