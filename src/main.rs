use clap::{Parser, Subcommand};
use rust_challenge_calibrator::ast_inspector::AstInspector;
use rust_challenge_calibrator::calibration::DifficultyCalibrator;
use rust_challenge_calibrator::screener::RepositoryScreener;
use rust_challenge_calibrator::task_spec::TaskSpecGenerator;
use rust_challenge_calibrator::verifier::DeterministicVerifier;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "challenge-calibrator")]
#[command(about = "Rust Challenge Screening, Calibration, and Deterministic Verifier Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Screen a repository and concept for benchmark challenge eligibility
    Screen {
        #[arg(short, long)]
        repo_path: PathBuf,
        #[arg(short, long, default_value = "async-concurrency")]
        concept: String,
    },
    /// Calibrate task difficulty against implementation volume
    Calibrate {
        #[arg(short, long, default_value = "sample_task")]
        task_name: String,
        #[arg(long, default_value_t = 8)]
        complexity: usize,
        #[arg(long, default_value_t = 3)]
        invariants: usize,
        #[arg(long, default_value_t = 35)]
        loc_boilerplate: usize,
        #[arg(long, default_value_t = 45)]
        loc_logic: usize,
    },
    /// Run deterministic verification and AST anti-cheat inspection on a patch
    Verify {
        #[arg(short, long)]
        task_id: String,
        #[arg(short, long)]
        patch_code: String,
    },
    /// Generate a realistic, non-prescriptive task description
    Describe,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Screen { repo_path, concept } => {
            let screener = RepositoryScreener::new(true);
            let report = screener.screen_repository(&repo_path, &concept);
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
        }
        Commands::Calibrate {
            task_name,
            complexity,
            invariants,
            loc_boilerplate,
            loc_logic,
        } => {
            let report = DifficultyCalibrator::evaluate_task(
                &task_name,
                complexity,
                invariants,
                loc_boilerplate,
                loc_logic,
            );
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
        }
        Commands::Verify { task_id, patch_code } => {
            let verifier = DeterministicVerifier::new(10);
            let outcome = verifier.verify_patch(&task_id, "", &patch_code);
            println!("{}", serde_json::to_string_pretty(&outcome).unwrap());
        }
        Commands::Describe => {
            let spec = TaskSpecGenerator::generate_sample_spec();
            println!("{}", serde_json::to_string_pretty(&spec).unwrap());
        }
    }
}
