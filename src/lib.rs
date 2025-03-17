use cucumber::{cli, event::{self}, writer::Normalized, Event};
use gherkin::{Feature, Step, StepType};
use handlebars::Handlebars;
use markdown2pdf::styling::StyleMatch;
use rust_embed::Embed;
use serde::Serialize;
use std::{collections::{HashMap, HashSet}, error::Error, fmt::Debug, hash::{DefaultHasher, Hash, Hasher}, sync::Arc};
use cucumber::event::Cucumber::{*};
use cucumber::event::Feature::{*};


#[derive(Embed)]
#[folder="templates"]
struct MarkdownTemplates;

#[derive(Debug,Clone,Serialize)]
pub enum StepState {
    Passed,
    Failed
} 
#[derive(Debug)]
pub struct CucumberReporter {
    features: HashSet<Arc<Feature>>,
    step_states: HashMap<u64,StepState>
}

type Result<T> = std::result::Result<T,Box<dyn Error>>;


#[derive(Serialize,Clone,Debug)]
struct FeatureRenderData {
    pub name: String,
    pub description: String
}


#[derive(Serialize,Clone,Debug)]
struct StepRenderData {
    step_type: StepType,
    step_state: StepState,
    step_template: String
}

#[derive(Serialize,Clone,Debug)]
struct ScenarioRenderData {
    pub name: String,
    pub description: String,
    pub steps: Vec<StepRenderData>,
}

impl CucumberReporter {

    pub fn new() -> Self {
        CucumberReporter {
            features: HashSet::new(),
            step_states: HashMap::new()
        }
    }

    pub async fn add_feature(&mut self,feature:Arc<Feature>) {
        if ! self.features.contains(&feature) {
            self.features.insert(feature.clone());
        }
    }

    pub async fn add_step(&mut self,step:Arc<Step>,state:StepState) {
        let mut hasher = DefaultHasher::new();
        step.hash(&mut hasher);
        let id = hasher.finish();
        self.step_states.insert(id, state);
    }

    pub async fn finish(&self) -> Result<()> {
        let mut templates = Handlebars::new();
        templates.register_embed_templates::<MarkdownTemplates>()?;

        let mut tokens = Vec::new();

        for feature in &self.features {

            let data = FeatureRenderData {
                name: feature.name.clone(),
                description: feature.description.clone().unwrap_or_default()
            };

            let feature_md = templates.render("feature.md", &data)?;
            let mut lexer =markdown2pdf::markdown::Lexer::new(feature_md);
            let mut feature_tokens = lexer.parse().unwrap();

            tokens.append(&mut feature_tokens);

            for scenario in &feature.scenarios {

                let data = ScenarioRenderData {
                    name: scenario.name.clone(),
                    description: scenario.description.clone().unwrap_or_default(),
                    steps: scenario.steps.iter().map(|s| StepRenderData {
                        step_type: s.ty,
                        step_template: s.value.clone(),
                        step_state: StepState::Passed
                    } ).collect()
                };

                let scenario_md = templates.render("scenario.md", &data)?;
                let mut lexer =markdown2pdf::markdown::Lexer::new(scenario_md);
                let mut scenario_tokens = lexer.parse().unwrap();

                tokens.append(&mut scenario_tokens);
            }
        };

         let pdf = markdown2pdf::pdf::Pdf::new(tokens,StyleMatch::default());
         let doc =pdf.render_into_document();
         doc.render_to_file("test.pdf")?;

        Ok(())
    }
}

impl Normalized for CucumberReporter {}

impl<W> cucumber::Writer<W> for CucumberReporter 
where W: 'static + Debug {
    type Cli = cli::Empty;

    async fn handle_event(
        &mut self,
        ev: cucumber::parser::Result<cucumber::Event<cucumber::event::Cucumber<W>>>,
        cli: &Self::Cli,
    )  {
        match ev {
            Ok(Event { value,.. }) => 
                match value  {
                    Feature(gherkin_feature, event) => {
                        self.add_feature(gherkin_feature).await;
                        match event {
                            event::Feature::Scenario(_gherkin_scenario,event) => {
                                match event.event {
                                    event::Scenario::Step(gherkin_step,event) => {
                                        match event {
                                            event::Step::Passed(_capture_locations, _location) => {
                                                self.add_step(gherkin_step,StepState::Passed).await;
                                            },
                                            event::Step::Failed(_capture_locations, _location, world, step_error) => {
                                                self.add_step(gherkin_step,StepState::Failed).await;
                                            }, 
                                            _ => {}
                                        }
                                    },
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    },
                    cucumber::event::Cucumber::Finished => {
                        self.finish().await;
                    },
                    _ => {}
                }
           _ => {}
        };
    }
}
