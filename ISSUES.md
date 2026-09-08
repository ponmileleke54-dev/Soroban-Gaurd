# Soroban Guard - Drips Contributor Backlog

## Issue #1: Implement Lint Rule SG002 - Storage TTL Extension Check
- **Difficulty:** Medium
- **Scope:** 1-week sprint
- **Description:** Implement a rule that scans persistent storage operations (`env.storage().persistent()`) and flags occurrences where no accompanying `.extend_ttl()` call exists.
- **Acceptance Criteria:**
  - Create `crates/soroban-guard-core/src/rules/sg002_ttl_check.rs`.
  - Implement `Rule` trait for `TtlExtensionRule`.
  - Add unit tests verifying detection on test contracts.

## Issue #2: Implement Lint Rule SG003 - Unbounded Vector Iteration Warning
- **Difficulty:** Easy
- **Scope:** 1-week sprint
- **Description:** Implement a rule that warns when smart contract code iterates over user-supplied `Vec` or `Map` parameters without an explicit length check.
- **Acceptance Criteria:**
  - Create `crates/soroban-guard-core/src/rules/sg003_unbounded_loop.rs`.
  - Flag `for` loops or iterator calls on `Vec` parameters lacking `.len()` checks.
  - Add unit tests.

## Issue #3: Implement JSON Output Formatting Flag (`--json`)
- **Difficulty:** Easy
- **Scope:** 1-week sprint
- **Description:** Add a `--json` output option to `soroban-guard-cli` so third-party tools and CI runners can parse diagnostics.
- **Acceptance Criteria:**
  - Update `Cli` struct in `main.rs` with `#[arg(long)] json: bool`.
  - Format output using `serde_json::to_string_pretty(&diagnostics)`.

## Issue #4: Add SARIF Exporter for GitHub Code Scanning
- **Difficulty:** Medium
- **Scope:** 1-week sprint
- **Description:** Implement a reporter that formats `Vec<Diagnostic>` into valid SARIF (Static Analysis Results Interchange Format) JSON.
- **Acceptance Criteria:**
  - Create `crates/soroban-guard-core/src/reporters/sarif.rs`.
  - Add `--format sarif` option to the CLI.

## Issue #5: Build Vulnerable Smart Contract Test Suite
- **Difficulty:** Easy
- **Scope:** 1-week sprint
- **Description:** Create a dedicated directory of sample Rust/Soroban contracts exhibiting known anti-patterns to serve as integration benchmarks.
- **Acceptance Criteria:**
  - Create `tests/fixtures/vulnerable_contracts/`.
  - Write sample contracts covering missing auth, unhandled TTL, and raw panic usage.