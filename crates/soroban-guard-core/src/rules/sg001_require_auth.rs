use syn::{visit::Visit, File, ItemFn, Visibility};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct RequireAuthRule;

impl Rule for RequireAuthRule {
    fn code(&self) -> &'static str {
        "SG001"
    }

    fn name(&self) -> &'static str {
        "Unprotected Mutable Contract Function"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = AuthVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct AuthVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
}

impl<'ast> Visit<'ast> for AuthVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        if matches!(node.vis, Visibility::Public(_)) {
            let fn_str = quote::quote!(#node).to_string();

            let has_require_auth = fn_str.contains("require_auth");
            let mutates_state = fn_str.contains("env . storage") || fn_str.contains("env . storage ( )");

            if mutates_state && !has_require_auth {
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG001".to_string(),
                    message: format!("Public function `{}` modifies state but lacks an authorization check (`e.require_auth()`).", node.sig.ident),
                    severity: Severity::Critical,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some("Add `user.require_auth();` before performing storage operations.".to_string()),
                });
            }
        }
        syn::visit::visit_item_fn(self, node);
    }
}