use std::error::Error;

use cucumber::{given, then, when, World};

type Result<T> = std::result::Result<T,Box<dyn Error>>;

#[derive(World,Clone,Default,Debug)]
pub struct ReporterWorld {}

#[given(expr="a fact")]
pub async fn given_a_fact(_world:&mut ReporterWorld) {
}

#[when(expr="something is executed")]
pub async fn when_something_is_executed(_world:&mut ReporterWorld) {
}

#[then(expr="the result is {word}")]
pub async fn then_result_is(_world:&mut ReporterWorld,outcome:String) -> Result<()> {
    if outcome ==  "oke" {
        Ok(())
    } else {
        Err("error".into())
    }
}