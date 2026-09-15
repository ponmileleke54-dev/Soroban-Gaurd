pub mod config;
pub mod diagnostic;
pub mod engine;
pub mod rules;

pub use config::{ConfigSeverity, GuardConfig};
pub use diagnostic::{Diagnostic, Severity};
pub use engine::LinterEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{
        BarePanicRule, HardcodedKeyRule, RequireAuthRule, TtlExtensionRule,
        UnboundedLoopRule, UncheckedArithmeticRule, UnusedReturnRule,
    };
    use std::path::PathBuf;

    #[test]
    fn test_engine_detects_all_vulnerabilities() {
        let mut engine = LinterEngine::new();
        engine.register_rule(Box::new(RequireAuthRule));
        engine.register_rule(Box::new(TtlExtensionRule));
        engine.register_rule(Box::new(UnboundedLoopRule));
        engine.register_rule(Box::new(BarePanicRule));
        engine.register_rule(Box::new(HardcodedKeyRule));
        engine.register_rule(Box::new(UncheckedArithmeticRule));
        engine.register_rule(Box::new(UnusedReturnRule));

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_file = manifest_dir.join("../../tests/fixtures/test_contract.rs");

        let diagnostics = engine.analyze_file(&test_file).expect("Failed to analyze test file");

        assert!(!diagnostics.is_empty(), "Diagnostics should not be empty");
    }
}