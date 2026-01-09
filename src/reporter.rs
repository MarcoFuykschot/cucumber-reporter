use cucumber::cli::Args;
use cucumber::event::Cucumber::*;
use cucumber::{
    Event,
    event::{self},
    writer::Normalized,
};
use gherkin::{Examples, Feature, GherkinEnv, Scenario, Step};
use handlebars::Handlebars;
use rust_embed::Embed;
use std::sync::Arc;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::render_types::*;

#[derive(Embed)]
#[folder = "templates"]
struct HtmlTemplates;

///  How to add to the default writer
/// ```rust
///   use cucumber::{World, WriterExt, writer::Basic};
///   use cucumber_reporter::CucumberReporter;
///
///   #[derive(World,Debug,Default)]
///   struct MyWorld;
///
///   MyWorld::cucumber()
///        .with_default_cli()
///        .with_writer(
///           Basic::stdout()
///                .summarized()
///                .tee::<MyWorld, _>(CucumberReporter::new()),
///        )
///        .run("features");
/// ```
#[derive(Debug)]
pub struct CucumberReporter {
    features: HashSet<Arc<Feature>>,
    orig_features: HashSet<Arc<Feature>>,
    step_states: HashMap<u64, StepState>,
    outlines: HashSet<u64>,
    nr_senarios: u32,
    nr_rules: u32,
    nr_steps: u32,
    nr_errors: u32,
    nr_skipped: u32,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

trait FeatureExt {
    fn filename(&self) -> String;
}

impl FeatureExt for Feature {
    fn filename(&self) -> String {
        format!("F{}.html", self.name.id())
    }
}

trait ToId: Hash {
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl ToId for Step {}
impl ToId for Examples {
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.span.hash(&mut hasher);
        hasher.finish()
    }
}
impl ToId for Scenario {
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.span.hash(&mut hasher);
        hasher.finish()
    }
}
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
            nr_errors: 0,
            nr_rules: 0,
            nr_senarios: 0,
            nr_skipped: 0,
            nr_steps: 0,
        }
    }

    fn add_feature(&mut self, feature: Arc<Feature>) {
        if !self.features.contains(&feature)
            && self.features.insert(feature.clone())
            && feature.scenarios.iter().any(|s| !s.examples.is_empty())
        {
            let org = Feature::parse_path(
                feature.path.clone().expect("Feature not found"),
                GherkinEnv::default(),
            )
            .expect("Failed to parse feature");
            self.orig_features.insert(Arc::new(org));
        }
    }

    fn add_step(&mut self, step: Arc<Step>, state: StepState) {
        self.step_states.insert(step.id(), state);
    }

    async fn finish(&mut self, args: &ReporterArgs) -> Result<()> {
        let mut templates = Handlebars::new();
        templates.register_embed_templates::<HtmlTemplates>()?;

        let mut index_data = Vec::new();

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
            let html = templates
                .render("page.html", &feature_html)
                .expect("Failed to render page.html");
            write_html_file(args, html, feature.filename())?;

            let all_scenarios = feature
                .scenarios
                .iter()
                .chain(feature.rules.iter().flat_map(|r| r.scenarios.iter()))
                .collect::<Vec<_>>();

            index_data.push(FeatureRenderStatsData {
                name: feature.name.clone(),
                link: feature.filename(),
                description: feature.description.clone().unwrap_or_default(),
                nr_scenarios: all_scenarios.len(),
                nr_rules: feature.rules.iter().count().into(),
                nr_steps: all_scenarios.iter().map(|s| s.steps.iter().count()).sum(),
                nr_errors: all_scenarios
                    .iter()
                    .map(|s| {
                        s.steps
                            .iter()
                            .filter(|st| {
                                self.step_states
                                    .get(&st.id())
                                    .is_some_and(|ss| ss == &StepState::Failed)
                            })
                            .count()
                    })
                    .sum(),
                nr_skipped: all_scenarios
                    .iter()
                    .map(|s| {
                        s.steps
                            .iter()
                            .filter(|st| {
                                self.step_states
                                    .get(&st.id())
                                    .is_none_or(|ss| ss == &StepState::NotRun)
                            })
                            .count()
                    })
                    .sum(),
            });
        }
        index_data.sort_by_key(|f| f.name.clone());
        let data = IndexRenderData {
            features: index_data.to_vec(),
        };
        let index_html = templates.render("index.html", &data)?;
        write_html_file(args, index_html, "index.html".to_string())?;
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
                .expect("Original feature not found");

            let org_scenario = org_feature
                .scenarios
                .iter()
                .find(|s| s.id() == scenario.id())
                .expect("Original scenario not found")
                .clone();

            if let Some(org_scenario) = org_scenario.cloned() && self.outline_processed(&org_scenario) {
                let example_ids = org_scenario
                    .examples
                    .iter()
                    .map(|e| e.id())
                    .collect::<Vec<_>>();

                let all_scenarios = feature
                    .scenarios
                    .iter()
                    .filter(|s| {
                        s.examples
                            .iter()
                            .map(|e| e.id())
                            .all(|f| example_ids.contains(&f))
                    })
                    .collect::<Vec<_>>();

                let data = OutlineRenderData {
                    name: org_scenario.name.clone(),
                    scenario_description: org_scenario.description.clone().unwrap_or_default(),
                    examples: org_scenario
                        .examples
                        .iter()
                        .map(|ex| {
                            let table = ex.table.clone().expect("table expected").clone();
                            ExampleRenderData {
                                name: ex.name.clone().unwrap_or_default(),
                                description: ex.description.clone().unwrap_or_default(),
                                headers: table.rows.first().expect("First row not found").clone(),
                                rows: table
                                    .rows
                                    .iter()
                                    .skip(1)
                                    .enumerate()
                                    .map(|(id, row)| {
                                        self.new_example_row(&all_scenarios, ex, id, row)
                                    })
                                    .collect::<Vec<_>>(),
                            }
                        })
                        .collect::<Vec<_>>(),
                    steps: org_scenario
                        .steps
                        .iter()
                        .map(|s| StepRenderData::new(s, StepState::NotRun))
                        .collect(),
                };
                let scenario_html = templates.render("outline.html", &data)?;
                Ok(scenario_html.to_string())
            } else {
                Ok(format!("Cannot find exampled scenario with {:?} span",scenario.span))
            }
        } else {
            let data = ScenarioRenderData {
                name: scenario.name.clone(),
                description: scenario.description.clone().unwrap_or_default(),
                steps: scenario
                    .steps
                    .iter()
                    .map(|s| {
                        StepRenderData::new(
                            s,
                            self.step_states
                                .get(&s.id())
                                .unwrap_or(&StepState::NotRun)
                                .clone(),
                        )
                    })
                    .collect(),
            };
            let scenario_html = templates.render("scenario.html", &data)?;
            Ok(scenario_html)
        }
    }

    fn new_example_row(
        &mut self,
        all_scenarios: &Vec<&Scenario>,
        ex: &Examples,
        id: usize,
        row: &Vec<String>,
    ) -> ExampleRowRenderData {
        let scenario_id = ex.position.line + 2 + id;
        let scenario = all_scenarios
            .iter()
            .find(|s| s.position.line == scenario_id)
            .expect("Scenario not found");
        let steps = scenario
            .steps
            .iter()
            .map(|step| {
                StepRenderData::new(
                    step,
                    self.step_states
                        .get(&step.id())
                        .unwrap_or(&StepState::NotRun)
                        .clone(),
                )
            })
            .collect::<Vec<_>>();
        let example_state = match steps
            .iter()
            .map(|step| step.step_state.clone())
            .collect::<Vec<_>>()
        {
            states if states.iter().any(|state| state == &StepState::Failed) => StepState::Failed,
            states if states.iter().all(|state| state == &StepState::Passed) => StepState::Passed,
            states if states.iter().any(|state| state == &StepState::NotRun) => StepState::NotRun,
            _ => todo!(),
        };
        ExampleRowRenderData {
            example: row.clone(),
            steps,
            example_state,
        }
    }

    fn process_scenario<W>(&mut self, event: event::RetryableScenario<W>) {
        self.nr_senarios += 1;
        if let event::Scenario::Step(gherkin_step, event) = event.event {
            self.nr_steps += 1;
            match event {
                event::Step::Passed(_capture_locations, _location) => {
                    self.add_step(gherkin_step.into(), StepState::Passed);
                }
                event::Step::Failed(_capture_locations, _location, _world, _step_error) => {
                    self.nr_errors += 1;
                    self.add_step(gherkin_step.into(), StepState::Failed);
                }
                event::Step::Skipped => {
                    self.nr_skipped += 1;
                }
                _ => {}
            }
        }
    }

    fn outline_processed(&mut self, scenario: &Scenario) -> bool {
        self.outlines.insert(scenario.id())
    }
}

fn write_html_file(args: &ReporterArgs, html: String, filename: String) -> Result<()> {
    let filename = if let Some(path) = &args.output_html_path {
        std::fs::create_dir_all(path)?;
        format!("{}/{}", path, filename)
    } else {
        format!("{}", filename)
    };
    std::fs::write(&filename, &html)?;
    Ok(())
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
                    self.add_feature(gherkin_feature.into());
                    match event {
                        event::Feature::Rule(_rule, event) => {
                            self.nr_rules += 1;
                            if let event::Rule::Scenario(_, event) = event {
                                self.process_scenario(event)
                            }
                        }
                        event::Feature::Scenario(_, event) => self.process_scenario(event),
                        _ => {}
                    }
                }
                cucumber::event::Cucumber::Finished => {
                    self.finish(cli).await.expect("Failed to finish reporter");
                }
                _ => {}
            }
        };
    }
}
