//! # Html reporter for cucumber
//! ## examples 
//! ```gherkin
#![doc = include_str!("../features/simple-feature.feature")]
//! ```
//! Will produce the following output
//! 
//! ![simple]("../assets/simple.png")
//! ```gherkin
#![doc = include_str!("../features/feature-with-rules.feature")]
//! ```
//! ```gherkin
#![doc = include_str!("../features/feature-with-outline.feature")]
//! ```

mod render_types;
mod reporter;
pub use reporter::CucumberReporter;