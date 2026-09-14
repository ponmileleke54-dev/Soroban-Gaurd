use std::fs;
use std::path::Path;
use syn::File;
use crate::diagnostic::Diagnostic;
use crate::rules::Rule;

pub struct LinterEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl LinterEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    pub fn analyze_file(&self, path: &Path) -> Result<Vec<Diagnostic>, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

        let ast: File = syn::parse_file(&content)
            .map_err(|e| format!("Failed to parse Rust AST in {:?}: {}", path, e))?;

        let path_str = path.to_string_lossy().to_string();
        let mut diagnostics = Vec::new();

        for rule in &self.rules {
            let mut rule_diags = rule.check(&ast, &path_str);
            diagnostics.append(&mut rule_diags);
        }

        Ok(diagnostics)
    }
}