use cucumber::cli::Args;
use cucumber::event::Cucumber::*;
use cucumber::{
    event::{self},
    writer::Normalized,
    Event,
};
use gherkin::{Examples, Feature, GherkinEnv, Scenario, Step};
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
struct HtmlTemplates;

#[derive(Debug, Clone, Serialize)]
pub enum StepState {
    Passed,
    Failed,
}
#[derive(Debug)]
pub struct CucumberReporter {
    features: HashSet<Arc<Feature>>,
    orig_features: HashSet<Arc<Feature>>,
    step_states: HashMap<u64, StepState>,
    outlines: HashSet<u64>,
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
    step_state: Option<StepState>,
    step_template: String,
}

/// Todo: gerkin languages
impl StepRenderData {
    fn new(step: &Step, state: Option<StepState>) -> Self {
        Self {
            is_and: step.keyword.to_lowercase().contains("and"),
            step_type: step.keyword.clone(),
            step_template: step.value.clone(),
            step_state: state,
        }
    }
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

#[derive(Serialize, Clone, Debug)]
struct ExampleRowRenderData {
    pub row: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
struct ExampleRenderData {
    pub name: String,
    pub description: String,
    pub headers: Vec<String>,
    pub rows: Vec<ExampleRowRenderData>,
}

#[derive(Serialize, Clone, Debug)]
struct OutlineRenderData {
    pub name: String,
    pub scenario_description: String,
    pub examples: Vec<ExampleRenderData>,
    pub steps: Vec<StepRenderData>,
}

trait ToId: Hash {
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl ToId for Step {}
impl ToId for Examples {}
impl ToId for Scenario {}
impl ToId for String {}

impl Default for CucumberReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl CucumberReporter {
    pub fn new() -> Self {
        CucumberReporter {
            features: HashSet::new(),
            orig_features: HashSet::new(),
            step_states: HashMap::new(),
            outlines: HashSet::new(),
        }
    }

    fn add_feature(&mut self, feature: Arc<Feature>) {
        if !self.features.contains(&feature)
            && self.features.insert(feature.clone())
            && feature.scenarios.iter().any(|s| !s.examples.is_empty())
        {
            let org =
                Feature::parse_path(feature.path.clone().unwrap(), GherkinEnv::default()).unwrap();
            self.orig_features.insert(Arc::new(org));
        }
    }

    fn add_step(&mut self, step: Arc<Step>, state: StepState) {
        self.step_states.insert(step.id(), state);
    }

    async fn finish(&mut self, args: &ReporterArgs) -> Result<()> {
        let mut templates = Handlebars::new();
        templates.register_embed_templates::<HtmlTemplates>()?;

        let features = self.features.clone();
        for feature in features {
            let mut scenarios = Vec::new();
            for scenario in &feature.scenarios {
                let scenario_html = self
                    .scenario_render(&templates, feature.clone(), scenario)
                    .await?
                    .clone();
                scenarios.push(scenario_html.to_string());
            }
            let mut rules = Vec::new();
            for rule in &feature.rules {
                let rules_html = self
                    .render_rule(&templates, feature.clone(), rule)
                    .await?
                    .clone();
                rules.push(rules_html.to_string());
            }

            let data = FeatureRenderData {
                name: feature.name.clone(),
                description: feature.description.clone().unwrap_or_default(),
                scenarios: scenarios.join(""),
                rules: rules.join(""),
            };
            let feature_html = templates.render("feature.html", &data)?;
            let html = templates.render("index.html", &feature_html).unwrap();
            let filename = if let Some(path) = &args.output_html_path {
                std::fs::create_dir_all(path).unwrap();
                format!(
                    "{}/{}.html",
                    path,
                    filenamify::filenamify(feature.name.clone())
                )
            } else {
                format!("{}.html", filenamify::filenamify(feature.name.clone()))
            };
            std::fs::write(filename, &html).unwrap();
        }
        Ok(())
    }

    async fn render_rule(
        &mut self,
        templates: &Handlebars<'_>,
        feature: Arc<Feature>,
        rule: &gherkin::Rule,
    ) -> Result<String> {
        let mut scenarios = Vec::new();
        for scenario in &rule.scenarios {
            let scenario_html = self
                .scenario_render(templates, feature.clone(), scenario)
                .await?;
            scenarios.push(scenario_html);
        }
        let data = RuleRenderData {
            name: rule.name.clone(),
            description: rule.description.clone().unwrap_or_default(),
            scenarios: scenarios.join(""),
        };
        let rules_html = templates.render("rule.html", &data)?;
        Ok(rules_html)
    }

    async fn scenario_render(
        &mut self,
        templates: &Handlebars<'_>,
        feature: Arc<Feature>,
        scenario: &gherkin::Scenario,
    ) -> Result<String> {
        if !scenario.examples.is_empty() {
            let org_feature = self
                .orig_features
                .iter()
                .find(|f| f.name == feature.name)
                .unwrap();

            let org_scenario = org_feature
                .scenarios
                .iter()
                .find(|s| s.span == scenario.span)
                .unwrap()
                .clone();

            if self.outline_processed(&org_scenario) {
                let data = OutlineRenderData {
                    name: org_scenario.name.clone(),
                    scenario_description: org_scenario.description.clone().unwrap_or_default(),
                    examples: scenario
                        .examples
                        .iter()
                        .map(|ex| {
                            let table = ex.table.clone().expect("table expected").clone();
                            ExampleRenderData {
                                name: ex.name.clone().unwrap_or_default(),
                                description: ex.description.clone().unwrap_or_default(),
                                headers: table.rows.first().unwrap().clone(),
                                rows: table
                                    .rows
                                    .iter()
                                    .skip(1)
                                    .enumerate()
                                    .map(|(id, row)| ExampleRowRenderData { row: row.clone() })
                                    .collect::<Vec<_>>(),
                            }
                        })
                        .collect::<Vec<_>>(),
                    steps: org_scenario
                        .steps
                        .iter()
                        .map(|s| StepRenderData::new(s, None))
                        .collect(),
                };
                let scenario_html = templates.render("outline.html", &data)?;
                Ok(scenario_html.to_string())
            } else {
                Ok("".to_string())
            }
        } else {
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
                        step_state: self.step_states.get(&s.id()).cloned(),
                    })
                    .collect(),
            };
            let scenario_html = templates.render("scenario.html", &data)?;
            Ok(scenario_html)
        }
    }

    fn process_scenario<W>(&mut self, event: event::RetryableScenario<W>) {
        if let event::Scenario::Step(gherkin_step, event) = event.event {
            match event {
                event::Step::Passed(_capture_locations, _location) => {
                    self.add_step(gherkin_step, StepState::Passed);
                }
                event::Step::Failed(_capture_locations, _location, _world, _step_error) => {
                    self.add_step(gherkin_step, StepState::Failed);
                }
                _ => {}
            }
        }
    }

    fn outline_processed(&mut self, scenario: &Scenario) -> bool {
        self.outlines.insert(scenario.id())
    }
}

#[derive(Args)]
pub struct ReporterArgs {
    #[arg(long = "output-html-path")]
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
        if let Ok(Event { value, .. }) = ev {
            match value {
                Feature(gherkin_feature, event) => {
                    self.add_feature(gherkin_feature);
                    match event {
                        event::Feature::Rule(_rule, event) => {
                            if let event::Rule::Scenario(_, event) = event {
                                self.process_scenario(event)
                            }
                        }
                        event::Feature::Scenario(_, event) => self.process_scenario(event),
                        _ => {}
                    }
                }
                cucumber::event::Cucumber::Finished => {
                    self.finish(cli).await.unwrap();
                }
                _ => {}
            }
        };
    }
}
