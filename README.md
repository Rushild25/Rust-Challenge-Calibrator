# Rust-Challenge-Calibrator 🦀🎯

> **High-Performance Rust Framework for Repository Eligibility Screening, Task Calibration, and Deterministic Behavior-Driven Test Verification for AI Coding Benchmarks.**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Verification](https://img.shields.io/badge/Verifiers-Deterministic%20BDD-brightgreen.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Testing](https://img.shields.io/badge/AST%20Inspection-Syn%20%2F%20Quote-red.svg)]()

---

## Overview

`Rust-Challenge-Calibrator` is a specialized engineering toolkit built in Rust to design, verify, and calibrate repository-level coding challenges for frontier AI model evaluation and software engineering benchmarks.

Modern coding evaluations suffer from two pervasive failure modes:
1. **Superficial Implementation Volume:** Challenges that test sheer typing volume, extensive boilerplate, or obscure framework configurations rather than core algorithmic and architectural problem solving.
2. **Weak & Non-Deterministic Verifiers:** Test suites prone to **false-positive passes** (where hardcoded constants, mock injections, or stubbed returns pass naive unit tests) or **flaky non-deterministic failures** caused by environmental race conditions.

`Rust-Challenge-Calibrator` provides an automated pipeline to screen candidate repositories, formulate precise non-prescriptive task descriptions, calibrate intrinsic problem difficulty, and execute hermetic, behavior-driven verification.

```
+--------------------------------------------------------------------------------------------------+
|                                    Rust-Challenge-Calibrator                                     |
+--------------------------------------------------------------------------------------------------+
|                                                                                                  |
|   +-----------------------+     +-----------------------+     +------------------------------+   |
|   | 1. Repository &       |     | 2. Task Specification |     | 3. Difficulty & Volume       |   |
|   |    Concept Screening  | --> |    & Acceptance Engine| --> |    Calibration Engine        |   |
|   | (Modularity, License, |     | (Non-prescriptive     |     | (Cyclomatic Depth vs Lines   |   |
|   |  Hermetic Isolation)  |     |  Behavior Contracts)  |     |  of Code / Boilerplate)      |   |
|   +-----------------------+     +-----------------------+     +------------------------------+   |
|                                                                              |                   |
|                                                                              v                   |
|   +------------------------------------------------------------------------------------------+   |
|   | 4. Deterministic Behavior-Driven Verification & Anti-Cheat Engine (AST Syn)              |   |
|   |    - AST inspection: detects trivial returns, mock stubbing, assert!(true) bypasses       |   |
|   |    - Fail-to-Pass (F2P) & Pass-to-Pass (P2P) dual-matrix execution                       |   |
|   |    - Fair agent failure diagnostics (distinguishing prompt ambiguity from model flaws)   |   |
|   +------------------------------------------------------------------------------------------+   |
|                                                                                                  |
+--------------------------------------------------------------------------------------------------+
```

---

## Core Capabilities

### 1. Repository & Concept Eligibility Screening
- Evaluates candidate open-source repositories against strict eligibility criteria:
  - **Hermetic Isolation:** Can the project build and test cleanly without external internet access or ambient host state?
  - **Dependency Hygiene:** Are dependencies pinned and minimal, preventing version drift?
  - **Architectural Modularity:** Are components cleanly decoupled so that a bug or feature has bounded blast radius?
  - **Defensible Complexity:** Does the challenge test genuine systems concepts (concurrency, memory management, protocol semantics) rather than mundane API plumbing?

### 2. Realistic, Non-Prescriptive Task Descriptions
- Automatically generates problem specifications modeled after senior engineering GitHub issues:
  - **Clear Acceptance Criteria:** Outlines inputs, expected outputs, invariants, and edge conditions.
  - **Zero Solution Leakage:** Avoids prescriptive internal implementation instructions, giving agents and human engineers freedom to architect clean alternate implementations.
  - **Environmental Guardrails:** Clear specification of runtime limits (time, memory, threads).

### 3. Difficulty Calibration (Difficulty vs. Volume)
- Quantitatively measures the ratio of **Architectural Depth** to **Implementation Volume**:
  $$\text{Calibration Score} = \frac{\text{Cyclomatic Complexity} \times \text{State Invariants}}{\text{Lines of Boilerplate Code}}$$
- Rejects challenges where difficulty is merely driven by typing 400 lines of repetitive DTO mappings.
- Flags challenges requiring deep reasoning in compact, critical-path modules (< 50 lines of precise algorithmic or concurrent code).

### 4. Deterministic Behavior-Driven Verification & Code Review
- **AST Anti-Cheat Inspection (`syn` parser):** Analyzes submitted code diffs to catch:
  - Weak test assertions (e.g., `assert!(true)`, broad catch blocks).
  - Hardcoded lookup tables or constant value sniffing.
  - Mock overrides in production execution paths.
- **Fail-to-Pass (F2P) & Pass-to-Pass (P2P) Matrix:** Verifies that the baseline test strictly fails before the patch and passes completely afterward, while existing unit suites remain 100% green.
- **Fair Agent Failure Analysis:** Structured error categorization:
  - `REASONING_DEFECT`: Model failed to account for state machine edge condition.
  - `CONTRACT_VIOLATION`: Output format or error variant mismatched.
  - `TEST_FLAKINESS`: Verifier race condition detected (flagged for verifier rework).

---

## Project Structure

```
Rust-Challenge-Calibrator/
├── Cargo.toml
├── Dockerfile
├── README.md
├── src/
│   ├── main.rs            # CLI entrypoint (clap)
│   ├── lib.rs             # Public API
│   ├── screener.rs        # Repository eligibility screening engine
│   ├── verifier.rs        # Deterministic BDD test verifier
│   ├── ast_inspector.rs   # syn-based AST anti-cheat & weak assertion detector
│   ├── calibration.rs     # Difficulty vs volume scoring metrics
│   └── task_spec.rs       # Non-prescriptive task description generator
├── tasks/                 # Sample challenge task definitions
│   └── async_channel_deadlock/
│       ├── task_description.md
│       ├── acceptance_criteria.md
│       ├── test_verifier.rs
│       └── reference_solution.rs
└── tests/
    └── verifier_integration_test.rs
```

---

## Quickstart

### Build from Source
```bash
cargo build --release
```

### 1. Screen a Repository for Challenge Eligibility
```bash
./target/release/challenge-calibrator screen --repo-path /path/to/target-repo --concept "async-concurrency"
```

Sample output:
```json
{
  "repo": "target-repo",
  "eligible": true,
  "scores": {
    "hermetic_build": 1.0,
    "dependency_hygiene": 0.95,
    "architectural_modularity": 0.88,
    "conceptual_depth": 0.92
  },
  "recommendation": "APPROVED: Suitable for high-signal concurrency challenge."
}
```

### 2. Calibrate Task Difficulty vs. Volume
```bash
./target/release/challenge-calibrator calibrate --task-path ./tasks/async_channel_deadlock
```

Sample output:
```json
{
  "task": "async_channel_deadlock",
  "genuine_difficulty": 8.7,
  "implementation_volume": 2.1,
  "calibration_verdict": "OPTIMAL_HIGH_DEPTH_LOW_VOLUME",
  "rationale": "High algorithmic and state-machine complexity concentrated in 38 lines of lock-free channel coordination."
}
```

### 3. Run Deterministic Verification on an Agent Patch
```bash
./target/release/challenge-calibrator verify \
    --task-path ./tasks/async_channel_deadlock \
    --patch ./tasks/async_channel_deadlock/reference_solution.rs \
    --strict-ast
```

Sample output:
```
[INFO] Running AST mutation and anti-cheat inspection...
[INFO] AST Clean: No hardcoded returns or assertion bypasses detected.
[INFO] Running Fail-to-Pass (F2P) suite: CONFIRMED FAILING on unpatched base.
[INFO] Applying patch and executing deterministic behavior verifier...
[SUCCESS] 14/14 behavior-driven tests passed. Zero flakiness across 10 randomized runs.
```

---

## License

This project is licensed under the [MIT License](LICENSE).
