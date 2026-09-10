#![doc = include_str!("../README.md")]

mod render_types;
mod reporter;
mod generator;

pub use reporter::CucumberReporter;
pub use generator::ReportGenerator;