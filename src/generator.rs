use std::path::PathBuf;

use anyhow::anyhow;
use gherkin::{Background, Table};
use tracing::{debug, info};
use typst::{
    foundations::{
        Array, Dict, IntoValue,
        Value::{self},
        dict,
    },
    syntax::{RootedPath, Source, VirtualPath, VirtualRoot},
};

use crate::info_types::{FeatureInfo, ReportInfo, StepInfo};
use crate::typst_world::{ReportGenerator, TypstTemplates};

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum ReportFormat {
    Pdf,
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

impl GherkinToDict for Background {
    fn to_dict(&self) -> Dict {
        let description = if let Some(description) = &self.description {
            description.replace("\n", "\r\n").into()
        }
        else {
            None
        };
        dict! {
            "name" => self.name.clone(),
            "description" => description,
            "steps" => Array::from_iter(self.steps.iter().map(|s| {
                StepInfo {
                    step: s.clone(),
                    result: None,
                }
                .to_dict()
                .into_value()
            })),
        }
    }
}

impl GherkinToDict for Table {
    fn to_dict(&self) -> Dict {
        dict! {
            "columns" => self.row_width(),
            "rows" => Array::from_iter(self.rows.iter().map(|row| {
                Array::from_iter(row.iter().map(|cell| cell.clone().into_value())).into_value()
            })),
        }
    }
}

impl ReportGenerator {
    pub fn generate_report(
        spec_files: &Vec<PathBuf>,
        result_files: &Option<Vec<PathBuf>>,
        output_document: &str,
        report_format: ReportFormat,
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
                        let env = gherkin::GherkinEnv::default();
                        let feature_input = match gherkin::Feature::parse_path(&spec_file, env) {
                            Ok(feature_parsed) => {
                                let feature_info = FeatureInfo {
                                    feature: feature_parsed,
                                    results: feature.into(),
                                };
                                feature_info.to_dict().into_value()
                            }
                            Err(e) => {
                                eprintln!("Error parsing feature file {spec_file:?}: {e}");
                                continue;
                            }
                        };
                        features_dict.push(feature_input);
                    };
                }
            }
        } else {
            info!("No JSON results file provided, generating report without test results.");
            for spec_file in spec_files {
                info!("Processing feature file: {:?}", spec_file);

                let env = gherkin::GherkinEnv::default();

                let feature_input = match gherkin::Feature::parse_path(&spec_file, env) {
                    Ok(feature_parsed) => {
                        let feature_info = FeatureInfo {
                            feature: feature_parsed,
                            results: None,
                        };
                        feature_info.to_dict().into_value()
                    }
                    Err(e) => {
                        eprintln!("Error parsing feature file {spec_file:?}: {e}");
                        continue;
                    }
                };
                features_dict.push(feature_input);
            }
        };

        // Load the JSON results
        let mut inputs = Dict::new();
        inputs.insert(
            "titlepage".into(),
            ReportInfo::default().to_dict().into_value(),
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

        let compile_pdf = typst::compile(&typst_world);

        for warning in &compile_pdf.warnings {
            debug!("Typst warning: {} ({:?})", warning.message, warning.span);
        }
        match compile_pdf.output {
            Ok(doc) => {
                let output_file_path = PathBuf::from(output_document);
                if let Ok(pdf_bytes) = typst_pdf::pdf(&doc, &typst_pdf::PdfOptions::default()) {
                    std::fs::write(output_file_path, pdf_bytes)?;
                } else {
                    eprintln!("Error generating PDF");
                }
            }
            Err(e) => {
                eprint!("Error compiling Typst template ");
                for error in e {
                    eprintln!("{} {:?}", error.message, error.hints);
                }
            }
        }
        Ok(())
    }
}
