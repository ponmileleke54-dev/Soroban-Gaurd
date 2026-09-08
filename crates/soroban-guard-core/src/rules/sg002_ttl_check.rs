use syn::{visit::Visit, File, ImplItemFn, Visibility};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct TtlExtensionRule;

impl Rule for TtlExtensionRule {
    fn code(&self) -> &'static str {
        "SG002"
    }

    fn name(&self) -> &'static str {
        "Missing Storage TTL Extension"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = TtlVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct TtlVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
}

impl<'ast> Visit<'ast> for TtlVisitor {
    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        if matches!(node.vis, Visibility::Public(_)) {
            let fn_str = quote::quote!(#node).to_string();

            let uses_persistent_or_instance = fn_str.contains("persistent") || fn_str.contains("instance");
            let extends_ttl = fn_str.contains("extend_ttl");

            if uses_persistent_or_instance && !extends_ttl {
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG002".to_string(),
                    message: format!(
                        "Public function `{}` accesses state storage without extending its Time-To-Live (`extend_ttl()`).",
                        node.sig.ident
                    ),
                    severity: Severity::Warning,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some("Call `env.storage().instance().extend_ttl(...)` to prevent state archival.".to_string()),
                });
            }
        }
        syn::visit::visit_impl_item_fn(self, node);
    }
}