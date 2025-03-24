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
    let assets = std::fs::read_dir("assets")?;

    let doc_path = if std::env::var("DOCS_RS").is_ok() {
        Path::new(std::env::var("CARGO_TARGET_DIR").unwrap().as_str()).to_path_buf()
    } else {
        let path = Path::new("target")
            .join("doc")
            .join("cucumber_reporter")
            .join("assets");
        fs::create_dir_all(path.clone())?;
        path
    };

    for asset in assets {
        let asset = asset?;
        std::fs::copy(asset.path(), doc_path.join(asset.file_name()))?;
    }

    Ok(())
}
