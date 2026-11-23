use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationReport {
    pub task_name: String,
    pub genuine_difficulty_score: f64,
    pub implementation_volume_score: f64,
    pub ratio: f64,
    pub verdict: String,
    pub rationale: String,
}

pub struct DifficultyCalibrator;

impl DifficultyCalibrator {
    /// Distinguishes between genuine architectural/algorithmic difficulty vs mere implementation volume.
    pub fn evaluate_task(
        task_name: &str,
        cyclomatic_complexity: usize,
        state_invariants: usize,
        loc_boilerplate: usize,
        loc_logic: usize,
    ) -> CalibrationReport {
        // High genuine difficulty comes from deep state machines, concurrency locks, and algorithmic invariants
        let genuine_difficulty = ((cyclomatic_complexity * 2 + state_invariants * 3) as f64)
            .min(10.0)
            .max(1.0);

        // Implementation volume is driven by boilerplate line count
        let implementation_volume = ((loc_boilerplate as f64 / 50.0) + (loc_logic as f64 / 80.0))
            .min(10.0)
            .max(1.0);

        let ratio = genuine_difficulty / implementation_volume;

        let (verdict, rationale) = if ratio >= 2.0 {
            (
                "OPTIMAL_HIGH_DEPTH_LOW_VOLUME".to_string(),
                "High conceptual difficulty concentrated in concise, critical-path logic. Ideal for frontier agent evaluation.".to_string(),
            )
        } else if ratio <= 0.6 {
            (
                "REJECTED_HIGH_VOLUME_LOW_DEPTH".to_string(),
                "Challenge tests excessive boilerplate typing rather than genuine engineering reasoning. Prone to token waste.".to_string(),
            )
        } else {
            (
                "ACCEPTABLE_BALANCED".to_string(),
                "Balanced challenge with moderate architectural depth and reasonable implementation surface.".to_string(),
            )
        };

        CalibrationReport {
            task_name: task_name.to_string(),
            genuine_difficulty_score: genuine_difficulty,
            implementation_volume_score: implementation_volume,
            ratio,
            verdict,
            rationale,
        }
    }
}
