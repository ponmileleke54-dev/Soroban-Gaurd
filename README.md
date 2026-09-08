# Soroban Guard (`soroban-guard`)

> **Automated Static Analysis & Security Linter for Soroban Smart Contracts on Stellar**

[![Build Status](https://github.com/ponmileleke54-dev/Soroban-Gaurd/workflows/CI/badge.svg)](https://github.com/ponmileleke54-dev/Soroban-Gaurd/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)

---

## Overview

**`soroban-guard`** is an open-source static analysis security linter tailored specifically for smart contracts built on the **Soroban** platform (Stellar's Rust-based WebAssembly smart contract engine). 

In decentralized finance (DeFi) and automated protocol ecosystems, subtle smart contract vulnerabilities—such as missing authorization checks, unhandled state Time-To-Live (TTL) extensions, or unconstrained storage loops—can lead to severe exploits, state freezes, or unexpected execution failures. 

`soroban-guard` acts as an automated security inspector. By directly parsing Rust source code into an Abstract Syntax Tree (AST) using `syn` and running deterministic static security visitors, `soroban-guard` detects vulnerability patterns during local development and inside CI/CD pipelines before code is ever compiled to WebAssembly (`.wasm`) or deployed on-chain.

---

## Key Features

- **Blazing Fast Parsing:** Evaluates contract code in under 100 milliseconds using direct AST traversal (`syn`/`quote`).
- **Soroban-Specific Lints:** Purpose-built rules detecting anti-patterns unique to the Soroban SDK and state architecture.
- **Actionable Diagnostics:** Precise error locations with severity rankings (*Critical*, *Warning*, *Info*) and actionable remediation suggestions.
- **Extensible Architecture:** Decoupled, trait-based rule runner enabling open-source contributors to independently build and register custom lint rules.
- **CI/CD Integration Ready:** Easily integrates into GitHub Actions, pre-commit hooks, and developer workflows.

---

## Project Workspace Architecture

`soroban-guard` is structured as a modular Cargo workspace divided into core library logic and a command-line interface (CLI):