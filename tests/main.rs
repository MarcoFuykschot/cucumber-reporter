use cucumber::{writer::Basic, World, WriterExt};
use cucumber_reporter::CucumberReporter;
use steps::ReporterWorld;

pub mod steps;

#[tokio::main]
async fn main() {
    ReporterWorld::cucumber()
        .with_default_cli()
        .with_writer(
            Basic::stdout()
                .summarized()
                .tee::<ReporterWorld, _>(CucumberReporter::new()),
        )
        .run("features")
        .await;
}
