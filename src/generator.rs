use std::path::PathBuf;

use anyhow::anyhow;
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use gherkin::{Background,Table};
use rust_embed::RustEmbed;
use tracing::{debug, info};
use typst::{
    Features, Library, LibraryExt, World,
    diag::{FileError, FileResult},
    foundations::{
        Array, Bytes, Datetime, Dict, Duration, IntoValue,
        Value::{self},
    },
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
};

use crate::generator::cucumber_json::ElementType;
#[allow(dead_code)]
#[allow(clippy::all, warnings)]
mod cucumber_json {
    include!(concat!(env!("OUT_DIR"), "/cucumber_json.rs"));
}

#[derive(RustEmbed)]
#[folder = "templates/typst/"]
struct TypstTemplates;

pub struct ReportGenerator {
    library: LazyHash<Library>,
    fontbook: FontBook,
    sources: Vec<Source>,
    fonts: Vec<typst::text::Font>,
}

impl ReportGenerator {
    fn typst_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<Datetime> {
        Datetime::from_ymd_hms(
            datetime.year(),
            datetime.month() as u8,
            datetime.day() as u8,
            datetime.hour() as u8,
            datetime.minute() as u8,
            datetime.second() as u8,
        )
    }
}

impl World for ReportGenerator {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    #[doc = " Metadata about all known fonts."]
    fn book(&self) -> &LazyHash<FontBook> {
        static LAZY_BOOK: std::sync::OnceLock<LazyHash<FontBook>> = std::sync::OnceLock::new();
        LAZY_BOOK.get_or_init(|| LazyHash::new(self.fontbook.clone()))
    }

    #[doc = " Get the file id of the main source file."]
    fn main(&self) -> FileId {
        self.sources.first().map(|s| s.id()).unwrap()
    }

    #[doc = " Try to access the specified file location as a source file."]
    fn source(&self, id: FileId) -> FileResult<Source> {
        let source = self.sources.iter().find(|s| s.id() == id).cloned();
        debug!("Accessing source file with id {:?}: {:?}", id, source);
        match source {
            Some(source) => {
                debug!("Accessed source file with id {:?}: {:?}", id, source);
                FileResult::Ok(source)
            }
            None => FileResult::Err(FileError::NotFound(
                "Source file not found".to_string().into(),
            )),
        }
    }

    #[doc = " Try to access the specified file."]
    #[doc = ""]
    #[doc = " For file locations for which [`source`](Self::source) succeeds, this"]
    #[doc = " should also succeed. The [`Bytes`] can be cheaply created as a view into"]
    #[doc = " an existing [`Source`] through [`Bytes::from_string`]."]
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.source(id).map(|s| {
            let s = s.text().to_string();
            Bytes::from_string(s)
        })
    }

    #[doc = " Try to access the font with the given index in the font book."]
    #[doc = ""]
    #[doc = " Note that the index is not guaranteed to be in bounds of the font book"]
    #[doc = " returned by this world\'s `book()` function. This is the case because"]
    #[doc = " this function may be invoked with indices from an outdated or different"]
    #[doc = " font book during incremental compilation validation."]
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    #[doc = " Get the current date."]
    #[doc = ""]
    #[doc = " If no offset is specified, the local date should be chosen. Otherwise,"]
    #[doc = " the UTC date should be chosen with the corresponding offset."]
    #[doc = ""]
    #[doc = " If this function returns `None`, Typst\'s `datetime` function will"]
    #[doc = " return an error."]
    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        match offset {
            None => Self::typst_datetime(Local::now()),
            Some(offset) => {
                let seconds = offset.seconds();
                let utc =
                    Utc::now() + chrono::Duration::microseconds((seconds * 1_000_000.0) as i64);
                Self::typst_datetime(utc)
            }
        }
    }
}

trait GherkinToDict {
    fn to_dict(&self) -> Dict;
}

struct FeatureInfo {
    pub feature: gherkin::Feature,
    pub results: cucumber_json::Feature,
}

#[derive(Debug)]
struct ScenarioInfo {
    pub scenario: gherkin::Scenario,
    pub results: Vec<cucumber_json::Element>,
}

impl FeatureInfo {
    pub fn outcome(&self) -> cucumber_json::Status {
        self.results
            .elements
            .iter()
            .flat_map(|s| s.steps.iter())
            .fold(cucumber_json::Status::Passed, |acc, step| {
                match (acc, step.result.status) {
                    (cucumber_json::Status::Failed, _) => cucumber_json::Status::Failed,
                    (_, cucumber_json::Status::Failed) => cucumber_json::Status::Failed,
                    (cucumber_json::Status::Skipped, _) => cucumber_json::Status::Skipped,
                    (_, cucumber_json::Status::Skipped) => cucumber_json::Status::Skipped,
                    _ => cucumber_json::Status::Passed,
                }
            })
    }
}

impl GherkinToDict for FeatureInfo {
    fn to_dict(&self) -> Dict {
        let mut dict = Dict::new();
        dict.insert("name".into(), Value::Str(self.feature.name.clone().into()));
        dict.insert(
            "description".into(),
            self.feature
                .description
                .clone()
                .map_or(Value::None, |d| Value::Str(d.into())),
        );
        if let Some(background)= &self.feature.background {
            dict.insert(
                "background".into(),
                Value::Dict(background.to_dict()),
            );
        } else {
            dict.insert("background".into(), Value::None);
        }
        dict.insert(
            "outcome".into(),
            Value::Str(format!("{:?}", self.outcome()).into()),
        );

        let scenarios_dict = Array::from_iter(self.feature.scenarios.iter().map(|info| {
            let scenario_info = ScenarioInfo {
                scenario: info.clone(),
                results: Vec::new(),
            };
            let results = self
                .results
                .elements
                .iter()
                .filter(|result| scenario_info.result_matches(result))
                .cloned()
                .collect();

            let scenario_info = ScenarioInfo {
                scenario: info.clone(),
                results,
            };
            scenario_info.to_dict().into_value()
        }));
        dict.insert("scenarios".into(), Value::Array(scenarios_dict));
        dict
    }
}

impl GherkinToDict for Background {
    fn to_dict(&self) -> Dict {
        let mut dict = Dict::new();
        dict.insert("name".into(), Value::Str(self.name.clone().into()));
        dict.insert(
            "description".into(),
            self.description
                .clone()
                .map_or(Value::None, |d| Value::Str(d.into())),
        );
        dict.insert(
            "steps".into(),
            Value::Array(Array::from_iter(self.steps.iter().map(|s| {
                StepInfo {
                    step: s.clone(),
                    result: None,
                }
                .to_dict()
                .into_value()
            }))),
        );
        dict
    }
}

impl ScenarioInfo {
    fn is_outline(&self) -> bool {
        self.scenario.keyword == "Scenario Outline" && !self.scenario.examples.is_empty()
    }

    fn result_matches(&self, result: &cucumber_json::Element) -> bool {
        if result.type_ != ElementType::Scenario {
            return false;
        }

        if self.is_outline() {
            self.scenario.examples.iter().any(|examples| {
                examples.table.as_ref().is_some_and(|table| {
                    table.rows.iter().skip(1).enumerate().any(|(row, _)| {
                        (examples.position.line + 2 + row) as f64 == result.line
                    })
                })
            })
        } else {
            result.line == self.scenario.position.line as f64
        }
    }

    fn get_result_for_step(&self, step: &gherkin::Step) -> Option<cucumber_json::Step> {
        if self.is_outline() {
            None
        } else {
            self.results
                .first()
                .and_then(|e| e.steps.iter().find(|s| s.name == step.value))
                .cloned()
        }
    }
}

impl GherkinToDict for ScenarioInfo {
    fn to_dict(&self) -> Dict {
        let mut dict = Dict::new();
        dict.insert("name".into(), Value::Str(self.scenario.name.clone().into()));
        dict.insert(
            "keyword".into(),
            Value::Str(self.scenario.keyword.clone().into()),
        );
        dict.insert(
            "description".into(),
            self.scenario
                .description
                .clone()
                .map_or(Value::None, |d| Value::Str(d.into())),
        );
        dict.insert(
            "steps".into(),
            Value::Array(Array::from_iter(self.scenario.steps.iter().map(|s| {
                StepInfo {
                    step: s.clone(),
                    result: self.get_result_for_step(s),
                }
                .to_dict()
                .into_value()
            }))),
        );
        dict.insert(
            "examples".into(),
            Value::Array(Array::from_iter(self.scenario.examples.iter().map(|ex| {
                ExampleInfo {
                    example: ex.clone(),
                    result: self.results.clone(),
                }
                .to_dict()
                .into_value()
            }))),
        );
        dict
    }
}

struct StepInfo {
    pub step: gherkin::Step,
    pub result: Option<cucumber_json::Step>,
}

struct ExampleInfo {
    pub example: gherkin::Examples,
    pub result: Vec<cucumber_json::Element>,
}

impl GherkinToDict for ExampleInfo {
    fn to_dict(&self) -> Dict {
        let mut dict = Dict::new();
        dict.insert(
            "name".into(),
            self.example
                .name
                .clone()
                .map_or_default(|name| Value::Str(name.into())),
        );

        if let Some(table) = &self.example.table {
            let colums = table.row_width();
            dict.insert("columns".into(), (colums + 1).into_value());

            let mut headers = table.rows.first().expect("At least one row").clone();
            headers.push("Outcome".into());

            let rows = [headers.clone()];

            debug!("{:#?}", self.example);
            debug!("{:#?}", self.result);

            dict.insert(
                "rows".into(),
                Value::Array(Array::from_iter(rows.iter().map(|row| {
                    Array::from_iter(row.iter().map(|cell| Value::Str(cell.clone().into())))
                        .into_value()
                }))),
            );
        } else {
            dict.insert("columns".into(), Value::None);
        }

        dict
    }
}

impl GherkinToDict for Table {
    fn to_dict(&self) -> Dict {
        let mut dict = Dict::new();
        dict.insert("columns".into(), self.row_width().into_value());

        dict.insert(
            "rows".into(),
            Value::Array(Array::from_iter(self.rows.iter().map(|row| {
                Array::from_iter(row.iter().map(|cell| Value::Str(cell.clone().into())))
                    .into_value()
            }))),
        );

        dict
    }
}

impl GherkinToDict for StepInfo {
    fn to_dict(&self) -> Dict {
        let mut dict = Dict::new();
        dict.insert(
            "keyword".into(),
            Value::Str(self.step.keyword.clone().into()),
        );
        dict.insert("text".into(), Value::Str(self.step.value.clone().into()));
        dict.insert(
            "outcome".into(),
            if let Some(result) = &self.result {
                Value::Str(format!("{:?}", result.result.status).into())
            } else {
                Value::None
            },
        );
        dict
    }
}

impl ReportGenerator {
    pub fn new(inputs: Dict) -> Self {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        typst_kit::fonts::embedded().for_each(|(font, info)| {
            book.push(info);
            fonts.push(font);
        });

        let features = Features::all();

        ReportGenerator {
            library: LazyHash::new(
                typst::Library::builder()
                    .with_features(features)
                    .with_inputs(inputs)
                    .build(),
            ),
            fontbook: book,
            sources: Vec::new(),
            fonts,
        }
    }

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
                    eprintln!(
                        "Error parsing feature file {feature_file_path:?}: {e}");
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
