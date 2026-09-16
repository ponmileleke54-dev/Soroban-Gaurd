# Soroban Guard

**Soroban Guard** is a fast, specialized static analysis linter designed specifically for Soroban smart contracts on the Stellar network. Built in Rust, it parses Abstract Syntax Trees (AST) using `syn` and `quote` to detect security vulnerabilities, missing authorization checks, expired TTL storage entries, and gas exhaustion risks before contracts are compiled to WebAssembly or deployed to Mainnet.

---

## Why Soroban Guard?

Unlike traditional EVM smart contracts, Soroban smart contracts operate under a unique execution model managed by the Stellar network:

1. **State Archival & TTL:** Storage entries in Soroban expire unless their Time-To-Live (TTL) is explicitly extended. Unextended entries risk archival, causing contract lockups.
2. **Authorization Engine:** Soroban utilizes native `require_auth()` and `require_auth_for_args()` checks rather than global msg.sender patterns.
3. **Resource Metering:** CPU and memory limits are strictly enforced on host function executions, making unbounded loops an immediate vector for transaction failure.

`soroban-guard` acts as an automated security companion in local development and CI/CD pipelines, flagging non-compliant patterns during the coding phase rather than post-deployment.

---

## Architecture Overview

`soroban-guard` is structured as a decoupled, modular Rust workspace:
soroban-guard/
├── crates/
│   ├── soroban-guard-core/  # Core analysis engine, AST visitors, & SARIF formatters
│   └── soroban-guard-cli/   # Command-line interface, argument parsing, & output renderers
├── .github/
│   └── workflows/ci.yml     # Workspace test runner and self-audit workflow
├── action.yml               # Composite GitHub Action for PR integration
├── RULES.md                 # Detailed security rule catalog with remediation examples
└── Cargo.toml               # Workspace manifest

### Core Components

* **`soroban-guard-core`**:
  * **Rule Trait (`rules::trait_rule::Rule`)**: Defines a unified interface for code analyzers (`code()`, `name()`, `check()`).
  * **AST Visitors (`syn::visit::Visit`)**: Recursively inspects functions, statements, function calls, macro invocations, and binary expressions across input target files.
  * **Engine (`LinterEngine`)**: Coordinates AST parsing, handles comment suppression filtering, applies configuration overrides, and generates structured `Diagnostic` objects.
  * **SARIF Engine (`sarif::SarifLog`)**: Formats analysis diagnostics into standard SARIF v2.1.0 JSON payloads.

* **`soroban-guard-cli`**:
  * Provides a lightweight command-line utility built with `clap` for parsing flags (`--format`, `--output`, `--config`).
  * Converts diagnostics into ANSI terminal text, JSON, GitHub workflow logging commands, or SARIF logs.

---

## Enforced Security Rules

`soroban-guard` enforces 8 security rules targeting common Soroban development pitfalls:

| Rule Code | Name | Severity | Description |
| :--- | :--- | :--- | :--- |
| **`SG001`** | Missing Authorization Check | **Critical** | Public `#[contractimpl]` functions modifying storage or transferring assets without calling `.require_auth()`. |
| **`SG002`** | Missing Storage TTL Extension | **Warning** | Storage mutations or reads missing corresponding `.extend_ttl()` calls, risking state expiration. |
| **`SG003`** | Unbounded Loop Iteration | **Warning** | Iterating over dynamic collections (`Vec`, `Map`) without enforcing maximum length checks. |
| **`SG004`** | Bare Panic Macro Usage | **Warning** | Calling generic `panic!()`, `todo!()`, or `unreachable!()` macros instead of `panic_with_error!`. |
| **`SG005`** | Hardcoded Addresses / Keys | **Critical** | Embedding static Stellar address strings (`G...`) or secret key prefixes (`S...`) into code. |
| **`SG006`** | Unchecked Arithmetic | **Warning** | Performing raw mathematical operations (`+`, `-`, `*`) without using checked arithmetic (`checked_add`). |
| **`SG007`** | Unused Storage Return Handle | **Warning** | Ignoring returned `Result` or `Option` values from storage reads (`try_get`). |
| **`SG008`** | Reentrancy State Mutation | **Critical** | Modifying contract storage (`storage().set()`) *after* triggering external cross-contract invocations. |

For deep-dive examples and remediation code samples for every rule, consult [RULES.md](RULES.md).

---

## Installation

### Prerequisites

* **Rust Toolchain:** Stable Rust installed via [rustup](https://rustup.rs/).
* **Cargo Package Manager**

### Building from Source

Clone the repository and build the CLI binary in release mode:

```bash
git clone [https://github.com/gamp/Soroban-Gaurd.git](https://github.com/gamp/Soroban-Gaurd.git)
cd Soroban-Gaurd
cargo build --release -p soroban-guard-cli