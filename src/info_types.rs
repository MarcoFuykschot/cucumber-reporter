use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typst::foundations::{dict, Array, Dict, IntoValue, Value};

use crate::generator::cucumber_json::ElementType;
use crate::generator::{cucumber_json, GherkinToDict};
use crate::typst_world::ReportGenerator;

#[derive(Debug, Clone)]
pub(crate) struct FeatureInfo {
    pub feature: gherkin::Feature,
    pub results: cucumber_json::Feature,
}

#[derive(Debug)]
pub(crate) struct ScenarioInfo {
    pub scenario: gherkin::Scenario,
    pub results: Vec<cucumber_json::Element>,
}

pub(crate) struct RuleInfo {
    pub rule: gherkin::Rule,
    pub feature: FeatureInfo,
}

pub(crate) struct StepInfo {
    pub step: gherkin::Step,
    pub result: Option<cucumber_json::Step>,
}

pub(crate) struct ExampleInfo {
    pub example: gherkin::Examples,
    pub result: Vec<cucumber_json::Element>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportInfo {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) sub_title: Option<String>,
    pub(crate) time_run: Option<DateTime<Utc>>,
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

    fn get_scenario_info(&self, info: &gherkin::Scenario) -> Value {
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
    }
}

impl GherkinToDict for FeatureInfo {
    fn to_dict(&self) -> Dict {
        let scenarios_dict = Array::from_iter(
            self.feature
                .scenarios
                .iter()
                .map(|info| self.get_scenario_info(info)),
        );

        dict! {
            "name" => self.feature.name.clone(),
            "description" => self.feature.description.clone(),
            "background" => self.feature.background.as_ref().map(|background| background.to_dict()),
            "outcome" => format!("{:?}", self.outcome()),
            "scenarios" => scenarios_dict,
            "path" => self.feature.path.clone().map_or(Value::None, |p| p.to_str().into_value()),
            "rules" => Array::from_iter( self.feature.rules.iter().map(|r| RuleInfo { rule: r.clone() , feature: self.clone() }.to_dict().into_value()))
        }
    }
}

impl GherkinToDict for RuleInfo {
    fn to_dict(&self) -> Dict {
        dict! {
            "name" => self.rule.name.clone(),
            "scenarios" => Array::from_iter(
                self.rule.scenarios.iter().map(|s| self.feature.get_scenario_info(s)))
        }
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
                    table
                        .rows
                        .iter()
                        .skip(1)
                        .enumerate()
                        .any(|(row, _)| (examples.position.line + 2 + row) as f64 == result.line)
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
        dict! {
            "name" => self.scenario.name.clone(),
            "keyword" => self.scenario.keyword.clone(),
            "description" => self.scenario.description.clone(),
            "steps" => Array::from_iter(self.scenario.steps.iter().map(|s| {
                StepInfo {
                    step: s.clone(),
                    result: self.get_result_for_step(s),
                }
                .to_dict()
                .into_value()
            })),
            "examples" => Array::from_iter(self.scenario.examples.iter().map(|ex| {
                ExampleInfo {
                    example: ex.clone(),
                    result: self.results.clone(),
                }
                .to_dict()
                .into_value()
            })),
        }
    }
}

impl GherkinToDict for ExampleInfo {
    fn to_dict(&self) -> Dict {
        if let Some(table) = &self.example.table {
            let colums = table.row_width();

            let mut headers = table.rows.first().expect("At least one row").clone();
            headers.push("Outcome".into());

            let rows = std::iter::once(headers)
                .chain(table.rows.iter().enumerate().skip(1).map(|(index, row)| {
                    let outcome = if let Some(result) = self.result.iter().find(|p| {
                        p.line == (index as f64 + self.example.position.line as f64) + 1.0
                    }) {
                        result
                            .steps
                            .iter()
                            .fold(cucumber_json::Status::Passed, |acc, step| {
                                match (acc, step.result.status) {
                                    (cucumber_json::Status::Failed, _) => {
                                        cucumber_json::Status::Failed
                                    }
                                    (_, cucumber_json::Status::Failed) => {
                                        cucumber_json::Status::Failed
                                    }
                                    (cucumber_json::Status::Skipped, _) => {
                                        cucumber_json::Status::Skipped
                                    }
                                    (_, cucumber_json::Status::Skipped) => {
                                        cucumber_json::Status::Skipped
                                    }
                                    _ => cucumber_json::Status::Passed,
                                }
                            })
                    } else {
                        cucumber_json::Status::Undefined
                    };
                    row.iter()
                        .chain([outcome.to_string()].iter())
                        .cloned()
                        .collect::<Vec<_>>()
                }))
                .collect::<Vec<_>>();

            dict! {
                "name" => self.example.name.clone(),
                "columns" => colums + 1,
                "rows" => Array::from_iter(rows.iter().map(|row| {
                    Array::from_iter(row.iter().map(|cell| cell.clone().into_value())).into_value()
                })),
            }
        } else {
            dict! {
                "name" => self.example.name.clone(),
                "columns" => Value::None,
            }
        }
    }
}

impl GherkinToDict for StepInfo {
    fn to_dict(&self) -> Dict {
        dict! {
            "keyword" => self.step.keyword.clone(),
            "text" => self.step.value.clone(),
            "outcome" => self.result.as_ref().map(|result| result.result.status.to_string()),
            "table" => self.step.table.as_ref().map_or_else(||Value::None, |t| t.to_dict().into_value())
        }
    }
}

impl Default for ReportInfo {
    fn default() -> Self {
        Self {
            title: "Gherking reporter".to_string(),
            author: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "Unknown".to_string()),
            sub_title: Default::default(),
            time_run: chrono::Local::now().to_utc().into(),
        }
    }
}

impl GherkinToDict for ReportInfo {
    fn to_dict(&self) -> Dict {
        dict! {
            "title" => self.title.clone(),
            "author" => self.author.clone(),
            "sub_title" => self.sub_title.clone().map_or_else(|| Value::None, |s| s.clone().into_value()),
            "time_run" => self.time_run.map_or_else(|| Value::None, |s| ReportGenerator::typst_datetime(  s.clone()).into_value() )
        }
    }
}
