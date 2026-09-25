#![doc = include_str!("../README.md")]

mod render_types;
mod reporter;
mod info_types;
mod typst_world;
mod generator;

pub use reporter::CucumberReporter;
pub use typst_world::ReportGenerator;