use std::{
    error::Error,
    fs::{self, File},
    io::Read,
    os::unix::fs::FileExt,
    path::Path,
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("DOCS_RS").is_err() {
        let mut readme = File::options().read(true).write(true).open("README.md")?;
        let mut content = String::new();
        readme.read_to_string(&mut content)?;

        let start = Regex::new("<!--CONTENT-START:(.*):(.*)-->")?;

        for capture in start.captures_iter(&content.clone()) {
            let filename = capture.get(1).unwrap().as_str();
            let code = capture.get(2).unwrap().as_str();
            let mut file = std::fs::File::open(filename)?;
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)?;

            let start_marker = format!("<!--CONTENT-START:{filename}:{code}-->");
            let end_marker = format!("<!--CONTENT-END:{filename}-->");

            if let Some(start) = content.find(&start_marker) {
                if let Some(end) = content.find(&end_marker) {
                    content.replace_range(
                        start..end + end_marker.len(),
                        format!(
                            "{start_marker}\r\n```{code}\r\n{file_content}\r\n```\r\n{end_marker}"
                        )
                        .as_str(),
                    )
                }
            }
        }
        readme.write_all_at(content.as_bytes(), 0)?;
    }

    let source =std::env::var("CARGO_SOURCE_DIR").unwrap_or_default();
    let assets_path = if std::env::var("DOCS_RS").is_ok() {
        Path::new(source.as_str())
    } else {
        Path::new(".")
    };

    let assets_path = assets_path.join("assets");

    let assets = std::fs::read_dir(assets_path)?;

    let target =std::env::var("CARGO_TARGET_DIR").unwrap_or_default();

    let doc_path = if std::env::var("DOCS_RS").is_ok() {
        Path::new(target.as_str())
    } else {
        Path::new("target")
    };
      let doc_path=  doc_path.join("doc")
            .join("cucumber_reporter")
            .join("assets");

     fs::create_dir_all(doc_path.clone())?;

    for asset in assets {
        let asset = asset?;
        let target_file = doc_path.join(asset.file_name());
        println!("cargo::warning={:?} {:?}",asset,target_file);
        std::fs::copy(asset.path(), target_file)?;
    }

    Ok(())
}
