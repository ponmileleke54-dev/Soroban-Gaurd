use syn::{visit::Visit, File, LitStr};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct HardcodedKeyRule;

impl Rule for HardcodedKeyRule {
    fn code(&self) -> &'static str {
        "SG005"
    }

    fn name(&self) -> &'static str {
        "Hardcoded Stellar Address or Private Key"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = KeyVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct KeyVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
}

impl<'ast> Visit<'ast> for KeyVisitor {
    fn visit_lit_str(&mut self, node: &'ast LitStr) {
        let value = node.value();

        // Detect Stellar Public Key (G...) or Secret Key (S...) consisting of 56 base32 characters
        if (value.starts_with('G') || value.starts_with('S')) && value.len() == 56 && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            let is_secret = value.starts_with('S');
            let severity = if is_secret { Severity::Critical } else { Severity::Warning };
            let key_type = if is_secret { "secret seed key" } else { "public account address" };

            self.diagnostics.push(Diagnostic {
                rule_code: "SG005".to_string(),
                message: format!(
                    "Found hardcoded Stellar {} embedded directly in contract source code.",
                    key_type
                ),
                severity,
                file_path: self.file_path.clone(),
                line: 1,
                column: 1,
                suggestion: Some(
                    "Pass account keys dynamically via `Address` parameters or environment storage instead of hardcoding.".to_string()
                ),
            });
        }
        syn::visit::visit_lit_str(self, node);
    }
}
