use std::path::{Path, PathBuf};

use anyhow::anyhow;
use tracing::{info, warn};
use typst::{
    foundations::{
        Array, Dict, IntoValue, dict,
        Value,
    },
    syntax::{RootedPath, Source, VirtualPath, VirtualRoot},
};

fn parse_error_feature_to_dict(spec_file: &Path, results: &cucumber_json::Feature) -> Option<Value> {
    let mut error_details = Vec::new();
    for element in &results.elements {
        let scenario_name = if element.name.trim().is_empty() {
            "Feature parse error".to_string()
        } else {
            element.name.clone()
        };

        for step in &element.steps {
            if let Some(message) = step.result.error_message.as_ref() {
                error_details.push((scenario_name.clone(), message.clone()));
            }
        }
    }

    if error_details.is_empty() {
        return None;
    }

    let error_message = error_details.first().map(|(_, message)| message.clone()).unwrap_or_default();

    let mut feature = Dict::new();
    feature.insert("name".into(), spec_file.file_name().map_or_else(|| "Feature parse error".to_string(), |name| name.to_string_lossy().to_string()).into_value());
    feature.insert("description".into(), Value::None);
    feature.insert("background".into(), Value::None);
    feature.insert("outcome".into(), "failed".into_value());
    feature.insert("scenarios".into(), Value::Array(Array::new()));
    feature.insert("path".into(), spec_file.to_string_lossy().into_value());
    feature.insert("rules".into(), Value::Array(Array::new()));
    feature.insert("tags".into(), Value::Array(Array::new()));
    feature.insert(
        "errors".into(),
        Array::from_iter(error_details.iter().map(|(scenario, message)| {
            dict! {
                "scenario" => scenario.clone(),
                "message" => message.clone(),
            }
            .into_value()
        })).into_value(),
    );
    feature.insert("error_message".into(), error_message.into_value());

    Some(feature.into_value())
}

use crate::info_types::{FeatureInfo, ReportInfo};
use crate::typst_world::{ReportGenerator, TypstTemplates};

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum ReportFormat {
    Pdf,
    #[value(help = "Experimental Typst HTML export; output may be incomplete")]
    Html,
}

#[allow(dead_code)]
#[allow(clippy::all, warnings)]
pub(crate) mod cucumber_json {
    include!(concat!(env!("OUT_DIR"), "/cucumber_json.rs"));
}

pub(crate) trait GherkinToDict {
    fn to_dict(&self) -> Dict;
}


fn feature_to_dict(spec_file: &Path, results: Option<cucumber_json::Feature>) -> Option<Value> {
    match gherkin::Feature::parse_path(spec_file, gherkin::GherkinEnv::default()) {
        Ok(feature) => Some(FeatureInfo { feature, results }.to_dict().into_value()),
        Err(error) => {
            eprintln!("Error parsing feature file {spec_file:?}: {error}");
            results
                .as_ref()
                .and_then(|results| parse_error_feature_to_dict(spec_file, results))
                .or_else(|| {
                    let message = format!("Could not parse feature file: {}", spec_file.display());
                    let mut feature = Dict::new();
                    feature.insert("name".into(), spec_file.file_name().map_or_else(|| "Feature parse error".to_string(), |name| name.to_string_lossy().to_string()).into_value());
                    feature.insert("description".into(), Value::None);
                    feature.insert("background".into(), Value::None);
                    feature.insert("outcome".into(), "failed".into_value());
                    feature.insert("scenarios".into(), Value::Array(Array::new()));
                    feature.insert("path".into(), spec_file.to_string_lossy().into_value());
                    feature.insert("rules".into(), Value::Array(Array::new()));
                    feature.insert("tags".into(), Value::Array(Array::new()));
                    let error_entry = dict! {
                        "scenario" => "Feature parse error",
                        "message" => message.clone(),
                    };
                    feature.insert(
                        "errors".into(),
                        Value::Array(Array::from_iter([error_entry.into_value()])),
                    );
                    feature.insert("error_message".into(), message.into_value());
                    Some(feature.into_value())
                })
        }
    }
}

impl ReportGenerator {
    pub fn generate_report(
        spec_files: &Vec<PathBuf>,
        result_files: &Option<Vec<PathBuf>>,
        output_document: &str,
        report_format: ReportFormat,
        run_info : &ReportInfo
    ) -> anyhow::Result<()> {
        let mut features_dict = Array::new();
        if let Some(results) = result_files {
            for json_results_file in results {
                info!("Processing JSON results file: {:?}", json_results_file);

                let file = std::fs::File::open(json_results_file)?;
                let reader = std::io::BufReader::new(file);
                let features: Vec<cucumber_json::Feature> = serde_json::from_reader(reader)?;

                // Process each feature
                for feature in features {
                    let feature_path = feature.uri.clone();

                    if let Some(spec_file) = spec_files
                        .iter()
                        .find(|spec_file| spec_file.ends_with(&feature_path))
                    {
                        info!("Processing feature file: {:?}", spec_file);
                        if let Some(feature_input) = feature_to_dict(spec_file, Some(feature)) {
                            features_dict.push(feature_input);
                        }
                    };
                }
            }
        } else {
            info!("No JSON results file provided, generating report without test results.");
            for spec_file in spec_files {
                info!("Processing feature file: {:?}", spec_file);

                if let Some(feature_input) = feature_to_dict(spec_file, None) {
                    features_dict.push(feature_input);
                }
            }
        };

        let mut inputs = Dict::new();
        inputs.insert(
            "runinfo".into(),
            run_info.to_dict().into_value(),
        );

        inputs.insert("features".into(), Value::Array(features_dict));
        let mut typst_world = ReportGenerator::new(inputs);

        let mut template_paths = TypstTemplates::iter()
            .filter(|path| path.ends_with(".typ"))
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        template_paths.sort_by_key(|path| path != "index.typ");

        for template_path in template_paths {
            let template = TypstTemplates::get(&template_path)
                .ok_or_else(|| anyhow!("Embedded Typst template not found: {template_path}"))?;
            let content = String::from_utf8(template.data.into_owned())?;
            let virtual_path = VirtualPath::new(&template_path)?;
            let template_id = RootedPath::new(VirtualRoot::Project, virtual_path).intern();
            typst_world.sources.push(Source::new(template_id, content));
        }

        match report_format {
            ReportFormat::Pdf => {
                let compilation = typst::compile(&typst_world);
                for warning in &compilation.warnings {
                    warn!("Typst warning: {} ({:?})", warning.message, warning.span);
                }
                match compilation.output {
                    Ok(doc) => {
                        let output_file_path = PathBuf::from(output_document);
                        if let Ok(pdf_bytes) =
                            typst_pdf::pdf(&doc, &typst_pdf::PdfOptions::default())
                        {
                            std::fs::write(output_file_path, pdf_bytes)?;
                        } else {
                            eprintln!("Error generating PDF");
                        }
                    }
                    Err(errors) => {
                        eprint!("Error compiling Typst template ");
                        for error in errors {
                            eprintln!("{} {:?}", error.message, error.hints);
                        }
                    }
                }
            }
            ReportFormat::Html => {
                let compilation = typst::compile::<typst_html::HtmlDocument>(&typst_world);
                for warning in &compilation.warnings {
                    warn!("Typst warning: {} ({:?})", warning.message, warning.span);
                }
                match compilation.output {
                    Ok(doc) => match typst_html::html(&doc, &typst_html::HtmlOptions::default()) {
                        Ok(html) => std::fs::write(output_document, html)?,
                        Err(errors) => {
                            eprintln!("Error exporting Typst HTML");
                            for error in errors {
                                eprintln!("{} {:?}", error.message, error.hints);
                            }
                        }
                    },
                    Err(errors) => {
                        eprint!("Error compiling Typst template ");
                        for error in errors {
                            eprintln!("{} {:?}", error.message, error.hints);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
