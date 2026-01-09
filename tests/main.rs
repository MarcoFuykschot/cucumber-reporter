use cucumber::{World, WriterExt, writer::Basic};
use cucumber_reporter::CucumberReporter;
use steps::test_steps::ReporterWorld;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer, fmt::format::{self, Format}, layer::SubscriberExt};

mod steps;

#[tokio::main]
async fn main() {

    ReporterWorld::cucumber()
        .with_default_cli()
        .configure_and_init_tracing(
        format::DefaultFields::new(),
        Format::default(),
        |fmt_layer| {
            tracing_subscriber::registry()
                .with(LevelFilter::INFO.and_then(fmt_layer))
        },
    )
        .with_writer(
            Basic::stdout()
                .summarized()
                .tee::<ReporterWorld, _>(CucumberReporter::new()),
        )
        .run("features")
        .await;
}
