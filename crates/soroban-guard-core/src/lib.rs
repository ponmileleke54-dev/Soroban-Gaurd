pub mod diagnostic;
pub mod engine;
pub mod rules;

pub use diagnostic::{Diagnostic, Severity};
pub use engine::LinterEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{
        RequireAuthRule, TtlExtensionRule, UnboundedLoopRule, BarePanicRule, HardcodedKeyRule, UncheckedArithmeticRule
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

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_file = manifest_dir.join("../../tests/fixtures/test_contract.rs");

        let diagnostics = engine.analyze_file(&test_file).expect("Failed to analyze test file");

        assert!(!diagnostics.is_empty(), "Diagnostics should not be empty");

        let rule_codes: Vec<String> = diagnostics.into_iter().map(|d| d.rule_code).collect();
        assert!(rule_codes.contains(&"SG001".to_string()), "Missing SG001 diagnostic");
        assert!(rule_codes.contains(&"SG002".to_string()), "Missing SG002 diagnostic");
        assert!(rule_codes.contains(&"SG003".to_string()), "Missing SG003 diagnostic");
        assert!(rule_codes.contains(&"SG004".to_string()), "Missing SG004 diagnostic");
        assert!(rule_codes.contains(&"SG006".to_string()), "Missing SG006 diagnostic");
    }
}