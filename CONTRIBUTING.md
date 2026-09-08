# Contributing to Soroban Guard

Thank you for contributing to `soroban-guard` through the Stellar Drips program!

## How to Add a New Lint Rule

1. Navigate to `crates/soroban-guard-core/src/rules/`.
2. Create a new file for your rule (e.g., `sg00X_my_rule.rs`).
3. Implement the `Rule` trait:
   ```rust
   use syn::File;
   use crate::diagnostic::Diagnostic;
   use crate::rules::trait_rule::Rule;

   pub struct MyRule;

   impl Rule for MyRule {
       fn code(&self) -> &'static str { "SG00X" }
       fn name(&self) -> &'static str { "Rule Name" }
       fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
           // Rule implementation
       }
   }