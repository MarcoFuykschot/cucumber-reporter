// No integration into code, a standaline tool to create a report
// includeing test results based on json cucumber format if present

use clap::Parser;
use cucumber_reporter::ReportGenerator;
use tracing::{error, info};


#[derive(Debug, clap::Parser)]
struct Args {
    /// The base path where the feature files are located.
    #[clap(long)]
    spec_base_path: String,
    /// Can be a glob pattern, for example: "target/**/*.json".
    #[clap(long)]
    json_results_path: String,
    #[clap(long, default_value = ".")]
    output_path: String,
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

    let glob_pattern = glob::Pattern::new(&args.json_results_path)?;

    for entry in glob::glob(&args.json_results_path)? {
        match entry {
            Ok(path) => {
                if path.is_file() && glob_pattern.matches_path(&path) {
                    info!("Found JSON result file: {:?}", path);
                    ReportGenerator::generate_report(&args.spec_base_path, &path, &args.output_path)?;
                }
            }
            Err(e) => error!("Error reading path: {:?}", e),
        }
    }
    Ok(())
}
