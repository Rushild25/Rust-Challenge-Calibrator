use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningReport {
    pub repository: String,
    pub is_eligible: bool,
    pub hermetic_build_score: f64,
    pub dependency_hygiene_score: f64,
    pub architectural_modularity_score: f64,
    pub conceptual_depth_score: f64,
    pub flags: Vec<String>,
    pub recommendation: String,
}

pub struct RepositoryScreener {
    pub strict_isolation: bool,
}

impl RepositoryScreener {
    pub fn new(strict_isolation: bool) -> Self {
        Self { strict_isolation }
    }

    pub fn screen_repository(&self, repo_path: &Path, concept: &str) -> ScreeningReport {
        let repo_name = repo_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut flags = Vec::new();
        let mut hermetic_score = 0.95;
        let mut dep_score = 0.90;
        let modularity_score = 0.88;
        let mut depth_score = 0.85;

        // Check for hermetic builds
        let cargo_toml = repo_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            flags.push("Missing Cargo.toml in root directory".to_string());
            hermetic_score -= 0.3;
        }

        // Screen concept depth
        match concept {
            "async-concurrency" | "memory-safety" | "distributed-consensus" => {
                depth_score = 0.95;
            }
            "crud-api" | "cli-boilerplate" => {
                flags.push("Concept lacks algorithmic or architectural depth (too volume-heavy)".to_string());
                depth_score = 0.35;
            }
            _ => {
                depth_score = 0.75;
            }
        }

        let is_eligible = hermetic_score >= 0.7 && dep_score >= 0.7 && depth_score >= 0.7;
        let recommendation = if is_eligible {
            format!("APPROVED: Eligible for benchmark challenge design on concept '{}'.", concept)
        } else {
            "REJECTED: Repository does not satisfy eligibility standards for high-signal evaluation.".to_string()
        };

        ScreeningReport {
            repository: repo_name,
            is_eligible,
            hermetic_build_score: hermetic_score,
            dependency_hygiene_score: dep_score,
            architectural_modularity_score: modularity_score,
            conceptual_depth_score: depth_score,
            flags,
            recommendation,
        }
    }
}
