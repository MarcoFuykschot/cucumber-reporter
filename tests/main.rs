use cucumber::{
    World, WriterExt,
    writer::{self, Basic},
};
use cucumber_reporter::CucumberReporter;
use steps::test_steps::ReporterWorld;
use tracing::level_filters::LevelFilter;

use tracing_subscriber::{
    Layer,
    fmt::format::{self, Format},
    layer::SubscriberExt,
};

mod steps;

#[tokio::main]
async fn main() {
    let file = std::fs::File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open("target/results.json")
        .unwrap();
    let tracer = tracing_subscriber::registry();

    ReporterWorld::cucumber()
        .with_default_cli()
        .configure_and_init_tracing(
            format::DefaultFields::new(),
            Format::default(),
            |fmt_layer| tracer.with(LevelFilter::INFO.and_then(fmt_layer)),
        )
        .with_writer(
            Basic::stdout()
                .summarized()
                .tee::<ReporterWorld, _>(writer::Json::for_tee(file))
                .tee::<ReporterWorld, _>(CucumberReporter::new())
                .normalized(),
        )
        .run("features")
        .await;
}
