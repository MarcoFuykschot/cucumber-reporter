use cucumber::cli::Args;
use cucumber::event::Cucumber::*;
use cucumber::{
    event::{self},
    writer::Normalized,
    Event,
};
use gherkin::{Feature, Step};
use handlebars::Handlebars;
use rust_embed::Embed;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

#[derive(Embed)]
#[folder = "templates"]
struct MarkdownTemplates;

#[derive(Debug, Clone, Serialize)]
pub enum StepState {
    Passed,
    Failed,
}
#[derive(Debug)]
pub struct CucumberReporter {
    features: HashSet<Arc<Feature>>,
    step_states: HashMap<u64, StepState>,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Clone, Debug)]
struct FeatureRenderData {
    pub name: String,
    pub description: String,
    pub scenarios: String,
    pub rules: String,
}

#[derive(Serialize, Clone, Debug)]
struct StepRenderData {
    is_and: bool,
    step_type: String,
    step_state: StepState,
    step_template: String,
}

#[derive(Serialize, Clone, Debug)]
struct RuleRenderData {
    pub name: String,
    pub description: String,
    pub scenarios: String,
}

#[derive(Serialize, Clone, Debug)]
struct ScenarioRenderData {
    pub name: String,
    pub description: String,
    pub steps: Vec<StepRenderData>,
}

trait StepExt: Hash {
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl StepExt for Step {}

impl CucumberReporter {
    pub fn new() -> Self {
        CucumberReporter {
            features: HashSet::new(),
            step_states: HashMap::new(),
        }
    }

    pub async fn add_feature(&mut self, feature: Arc<Feature>) {
        if !self.features.contains(&feature) {
            self.features.insert(feature.clone());
        }
    }

    pub async fn add_step(&mut self, step: Arc<Step>, state: StepState) {
        self.step_states.insert(step.id(), state);
    }

    pub async fn finish(&self, args: &ReporterArgs) -> Result<()> {
        let mut templates = Handlebars::new();
        templates.register_embed_templates::<MarkdownTemplates>()?;

        for feature in &self.features {

            let mut scenarios = Vec::new();
            for scenario in &feature.scenarios {
                let scenario_html = self.scenario_render(&templates, scenario)?;
                scenarios.push(scenario_html);
            }
            let mut rules = Vec::new();
            for rule in &feature.rules {
                let mut scenarios = Vec::new();
                for scenario in &rule.scenarios {
                    let scenario_html = self.scenario_render(&templates, scenario)?;
                    scenarios.push(scenario_html);
                }
                let data = RuleRenderData{
                    name : rule.name.clone(),
                    description: rule.description.clone().unwrap_or_default(),
                    scenarios : scenarios.join("")
                };
                let rules_html = templates.render("rule.html", &data)?;
                rules.push(rules_html);
            }

            let data = FeatureRenderData {
                name: feature.name.clone(),
                description: feature.description.clone().unwrap_or_default(),
                scenarios: scenarios.join(""),
                rules: rules.join(""),
            };
            let feature_html = templates.render("feature.html", &data)?;
            let html = templates.render("index.html", &feature_html).unwrap();
            let filename = if let Some(path) = &args.output_html_path  {
                format!("{}/{}.html", path, filenamify::filenamify(feature.name.clone()))
            } else {
                format!("{}.html", filenamify::filenamify(feature.name.clone()))
            }; 
            std::fs::write(filename, &html).unwrap();
        }
        Ok(())
    }

    fn scenario_render(
        &self,
        templates: &Handlebars<'_>,
        scenario: &gherkin::Scenario,
    ) -> Result<String> {
        let data = ScenarioRenderData {
            name: scenario.name.clone(),
            description: scenario.description.clone().unwrap_or_default(),
            steps: scenario
                .steps
                .iter()
                .map(|s| StepRenderData {
                    is_and: s.keyword.contains("And"),
                    step_type: s.keyword.clone(),
                    step_template: s.value.clone(),
                    step_state: self.step_states.get(&s.id()).unwrap().clone(),
                })
                .collect(),
        };
        let scenario_html = templates.render("scenario.html", &data)?;
        Ok(scenario_html)
    }

    async fn process_scenario<W>(&mut self, event: event::RetryableScenario<W>) {
        match event.event {
            event::Scenario::Step(gherkin_step, event) => match event {
                event::Step::Passed(_capture_locations, _location) => {
                    self.add_step(gherkin_step, StepState::Passed).await;
                }
                event::Step::Failed(_capture_locations, _location, _world, _step_error) => {
                    self.add_step(gherkin_step, StepState::Failed).await;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Args)]
pub struct ReporterArgs {
    #[arg()]
    pub output_html_path: Option<String>,
}

impl Normalized for CucumberReporter {}

impl<W> cucumber::Writer<W> for CucumberReporter
where
    W: 'static + Debug,
{
    type Cli = ReporterArgs;

    async fn handle_event(
        &mut self,
        ev: cucumber::parser::Result<cucumber::Event<cucumber::event::Cucumber<W>>>,
        cli: &Self::Cli,
    ) {
        match ev {
            Ok(Event { value, .. }) => match value {
                Feature(gherkin_feature, event) => {
                    self.add_feature(gherkin_feature).await;
                    match event {
                        event::Feature::Rule(_rule, event) => match event {
                            event::Rule::Scenario(_gherkin_scenario, event) => {
                                self.process_scenario(event).await
                            }
                            _ => {}
                        },
                        event::Feature::Scenario(_gherkin_scenario, event) => {
                            self.process_scenario(event).await
                        }
                        _ => {}
                    }
                }
                cucumber::event::Cucumber::Finished => {
                    let _ = self.finish(cli).await;
                }
                _ => {}
            },
            _ => {}
        };
    }
}
