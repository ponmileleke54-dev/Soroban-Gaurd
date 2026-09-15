use crate::config::{ConfigSeverity, GuardConfig};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::Rule;
use syn::File;
use std::fs;
use std::path::Path;

pub struct LinterEngine {
    rules: Vec<Box<dyn Rule>>,
    config: GuardConfig,
}

impl Default for LinterEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LinterEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            config: GuardConfig::default(),
        }
    }

    pub fn with_config(config: GuardConfig) -> Self {
        Self {
            rules: Vec::new(),
            config,
        }
    }

    pub fn register_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    pub fn analyze_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Diagnostic>, String> {
        let path_ref = path.as_ref();
        let content = fs::read_to_string(path_ref)
            .map_err(|e| format!("Failed to read file {:?}: {}", path_ref, e))?;

        let ast: File = syn::parse_str(&content)
            .map_err(|e| format!("Failed to parse Rust AST for {:?}: {}", path_ref, e))?;

        let file_str = path_ref.to_string_lossy().to_string();
        let mut raw_diagnostics = Vec::new();

        for rule in &self.rules {
            let rule_code = rule.code();

            // Check if rule is disabled in config
            if let Some(ConfigSeverity::Ignore) = self.config.rules.get(rule_code) {
                continue;
            }

            let mut diagnostics = rule.check(&ast, &file_str);

            // Apply severity overrides from config
            if let Some(override_severity) = self.config.rules.get(rule_code) {
                for diag in &mut diagnostics {
                    match override_severity {
                        ConfigSeverity::Critical => diag.severity = Severity::Critical,
                        ConfigSeverity::Warning => diag.severity = Severity::Warning,
                        ConfigSeverity::Info => diag.severity = Severity::Info,
                        ConfigSeverity::Ignore => {}
                    }
                }
            }

            raw_diagnostics.extend(diagnostics);
        }

        Ok(raw_diagnostics)
    }
}