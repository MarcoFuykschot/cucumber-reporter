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
        dict! {
            "name" => self.name.clone(),
            "description" => self.description.clone(),
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
        spec_base_path: &str,
        json_results_file: &PathBuf,
        output_path: &str,
    ) -> anyhow::Result<()> {
        // Load the JSON results
        let file = std::fs::File::open(json_results_file)?;
        let reader = std::io::BufReader::new(file);
        let features: Vec<cucumber_json::Feature> = serde_json::from_reader(reader)?;

        let mut inputs = Dict::new();

        inputs.insert(
            "titlepage".into(),
            ReportInfo::default().to_dict().into_value(),
        );

        let mut features_dict = Array::new();
        // Process each feature
        for feature in features {
            let feature_path = feature.uri.clone();
            let feature_file_path = std::path::Path::new(spec_base_path).join(&feature_path);
            info!("Processing feature file: {:?}", feature_file_path);

            let env = gherkin::GherkinEnv::default();

            let feature_input = match gherkin::Feature::parse_path(&feature_file_path, env) {
                Ok(feature_parsed) => {
                    let feature_info = FeatureInfo {
                        feature: feature_parsed,
                        results: feature,
                    };
                    feature_info.to_dict().into_value()
                }
                Err(e) => {
                    eprintln!("Error parsing feature file {feature_file_path:?}: {e}");
                    continue;
                }
            };
            features_dict.push(feature_input);
        }

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
                let output_file_path = PathBuf::from(output_path).join("report.pdf");
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
