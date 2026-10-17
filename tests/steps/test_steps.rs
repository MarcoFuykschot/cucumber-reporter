use std::error::Error;

use cucumber::{World, given, then, when};
use gherkin::Step;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(World, Clone, Default, Debug)]
pub struct ReporterWorld {}

#[given(expr = "a fact")]
#[given(expr = "a other fact")]
#[given(expr = "een ander feit")]
#[given(expr = "een feit")]
pub async fn given_a_fact(_world: &mut ReporterWorld) {}

#[given(expr = "some facts")]
#[given(expr = "wat feiten")]
pub async fn given_some_fact(_world: &mut ReporterWorld,step:&Step) -> Result<()> {
    if step.table().is_some() {
        Ok(())
    } else {
        Err("no table".into())
    }
}

#[given(expr = "a fact with {string}")]
#[given(expr = "een feit met {string}")]
pub async fn given_a_fact_with(_world: &mut ReporterWorld, value: String) -> Result<()> {
    if value == "Value 2" {
        Err("expected failed".into())
    } else {
        Ok(())
    }
}

#[when(expr = "something is executed")]
#[when(expr = "iets wordt uitgevoerd")]
pub async fn when_something_is_executed(_world: &mut ReporterWorld) {}

#[then(expr = "the result is {word}")]
#[then(expr = "het resultaat is {word}")]
pub async fn then_result_is(_world: &mut ReporterWorld, outcome: String) -> Result<()> {
    if outcome == "oke" {
        Ok(())
    } else {
        Err("expected error".into())
    }
}
