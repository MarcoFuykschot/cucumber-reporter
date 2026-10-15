// No integration into code, a standaline tool to create a report
// includeing test results based on json cucumber format if present

use clap::Parser;
use cucumber_reporter::{ReportFormat, ReportGenerator, ReportInfo};
use tracing::{debug, info};

#[derive(Debug, clap::Parser)]
struct Args {
    /// The search path where the features files are located.
    #[clap(long)]
    spec_path: String,
    /// Resuls be a glob pattern, for example: "target/**/*.json".
    #[clap(long)]
    json_results_path: Option<String>,
    #[clap(long, default_value = "report.pdf")]
    output_document: String,
    /// The format of the generated report.
    #[clap(long, default_value = "pdf")]
    report_format: ReportFormat,
    /// run information in yaml format
    #[clap(long)]
    run_information: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // init tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_ansi(false)
        .init();

    info!("Generating report from spec path: {}", args.spec_path);
    let spec_files = glob::glob(&args.spec_path)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    debug!(?spec_files, "Found spec files");

    let json_results_files = if let Some(json_results_path) = args.json_results_path {
        glob::glob(&json_results_path)
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    debug!(?json_results_files, "Found JSON results files");

    let run_info = if let Some(run_info_path) = args.run_information {
        let run_info_yaml = std::fs::read_to_string(run_info_path)?;
        serde_yaml::from_str(&run_info_yaml)?
    } else {
        ReportInfo::default()
    };

    ReportGenerator::generate_report(
        &spec_files,
        &if json_results_files.is_empty() {
            None
        } else {
            Some(json_results_files)
        },
        &args.output_document,
        args.report_format,
        &run_info,
    )?;

    Ok(())
}
