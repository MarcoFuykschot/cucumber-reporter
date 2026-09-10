use minify_html::{Cfg, minify};
use regex::Regex;
use std::{error::Error, fs::File, io::Read, os::unix::fs::FileExt};

fn main() -> Result<(), Box<dyn Error>> {
    generate_cucumber_json_types()?;

    if std::env::var("DOCS_RS").is_err() {
        let mut readme = File::options().read(true).write(true).open("README.md")?;
        let mut content = String::new();
        readme.read_to_string(&mut content)?;

        let start = Regex::new("<!--CONTENT-START:(.*):(.*)-->")?;

        for capture in start.captures_iter(&content.clone()) {
            let filename = capture.get(1).unwrap().as_str();
            let code = capture.get(2).unwrap().as_str().to_string();
            let mut file = std::fs::File::open(filename)?;
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)?;

            let start_marker = if code.is_empty() {
                format!("<!--CONTENT-START:{filename}:-->")
            } else {
                format!("<!--CONTENT-START:{filename}:{code}-->")
            };

            let file_content = if code.is_empty() {
                let mut cfg = Cfg::new();
                cfg.minify_css = true;
                cfg.keep_closing_tags = true;
                cfg.keep_html_and_head_opening_tags = true;
                let minified = minify(file_content.as_bytes(), &cfg);
                String::from_utf8(minified)?
            } else {
                file_content
            };

            let end_marker = format!("<!--CONTENT-END:{filename}-->");
            let replace_value = if code.is_empty() {
                format!("{start_marker}\r\n{file_content}\r\n{end_marker}")
            } else {
                format!("{start_marker}\r\n```{code}\r\n{file_content}\r\n```\r\n{end_marker}")
            };

            if let Some(start) = content.find(&start_marker)
                && let Some(end) = content.find(&end_marker)
            {
                content.replace_range(start..end + end_marker.len(), replace_value.as_str())
            };
        }
        readme.write_all_at(content.as_bytes(), 0)?;
    }
    Ok(())
}

fn generate_cucumber_json_types() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=schemas/canonical.json");

    let output = std::env::var("OUT_DIR")?;
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(include_str!(
        "schemas/canonical.json"
    ))?;
    let mut types = typify::TypeSpace::default();
    types.add_root_schema(schema)?;

    std::fs::write(
        std::path::Path::new(&output).join("cucumber_json.rs"),
        types.to_stream().to_string(),
    )?;
    Ok(())
}
