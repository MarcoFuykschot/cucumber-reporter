use gherkin::Step;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub(crate) struct FeatureRenderData {
    pub name: String,
    pub description: String,
    pub scenarios: String,
    pub rules: String,
}

#[derive(Serialize, Clone, Debug,Default)]
pub(crate) struct FeatureRenderStatsData {
    pub name: String,
    pub link: String,
    pub description:String,
    pub nr_scenarios: usize,
    pub nr_rules: usize,
    pub nr_steps: usize,
    pub nr_errors: usize,
    pub nr_skipped: usize,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct IndexRenderData {
    pub features : Vec<FeatureRenderStatsData>
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct StepRenderData {
    pub step_type: String,
    pub step_state: StepState,
    pub step_template: String,
    pub step_table: Option<Vec<Vec<String>>>
}

/// Todo: gerkin languages
impl StepRenderData {
    pub(crate) fn new(step: &Step, state: StepState) -> Self {
        Self {
            step_type: step.keyword.trim().to_string(),
            step_template: step.value.clone(),
            step_state: state,
            step_table: step.table.as_ref().map(|t| t.rows.clone())
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct RuleRenderData {
    pub name: String,
    pub description: String,
    pub scenarios: String,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct ScenarioRenderData {
    pub name: String,
    pub description: String,
    pub steps: Vec<StepRenderData>,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct ExampleRowRenderData {
    pub example: Vec<String>,
    pub steps: Vec<StepRenderData>,
    pub example_state: StepState,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct ExampleRenderData {
    pub name: String,
    pub description: String,
    pub headers: Vec<String>,
    pub rows: Vec<ExampleRowRenderData>,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct OutlineRenderData {
    pub name: String,
    pub scenario_description: String,
    pub examples: Vec<ExampleRenderData>,
    pub steps: Vec<StepRenderData>,
}

/// different step states
#[derive(Debug, Clone, Serialize,PartialEq)]
pub(crate) enum StepState {
    Passed,
    Failed,
/// When a step is skipped,not run or a template step from outline
/// also if a previous step has failed
    NotRun,
}

