#![doc = include_str!("../README.md")]

mod info_types;
mod typst_world;
mod generator;

pub use typst_world::ReportGenerator;
pub use generator::ReportFormat;
pub use info_types::ReportInfo;