use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpecification {
    pub task_id: String,
    pub title: String,
    pub background: String,
    pub behavioral_requirements: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub verification_protocol: String,
}

pub struct TaskSpecGenerator;

impl TaskSpecGenerator {
    pub fn generate_sample_spec() -> TaskSpecification {
        TaskSpecification {
            task_id: "rust-tokio-deadlock-01".to_string(),
            title: "Fix Mutex Contention in Async Dispatch Worker Queue".to_string(),
            background: "The worker queue occasionally stalls under high burst traffic (>10,000 tasks/s) due to mutex acquisition inversion between dispatch and acknowledgment routines.".to_string(),
            behavioral_requirements: vec![
                "Guarantee non-blocking enqueue operations without acquiring the internal dispatch mutex across await points.".to_string(),
                "Preserve FIFO task ordering under concurrent multi-producer, single-consumer configurations.".to_string(),
                "Ensure zero dropped tasks when workers trigger graceful shutdown signals.".to_string(),
            ],
            acceptance_criteria: vec![
                "All tests in `tests/concurrency_stress.rs` pass across 100 consecutive randomized iterations.".to_string(),
                "Zero deadlock events detected under 30-second continuous load test with 64 concurrent producers.".to_string(),
                "Memory consumption remains bounded to < 128MB with no unbounded channel buffers.".to_string(),
            ],
            out_of_scope: vec![
                "Refactoring the external public API interface.".to_string(),
                "Modifying consumer dispatch rate limiting configs.".to_string(),
            ],
            verification_protocol: "Hermetic Docker container execution with pinned Rust 1.75 toolchain and deterministic seed execution.".to_string(),
        }
    }
}
