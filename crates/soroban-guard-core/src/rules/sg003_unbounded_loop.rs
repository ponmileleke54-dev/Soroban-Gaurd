use syn::{visit::Visit, File, ImplItemFn, Visibility};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct UnboundedLoopRule;

impl Rule for UnboundedLoopRule {
    fn code(&self) -> &'static str {
        "SG003"
    }

    fn name(&self) -> &'static str {
        "Unbounded Vector or Map Iteration"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = LoopVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct LoopVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
}

impl<'ast> Visit<'ast> for LoopVisitor {
    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        if matches!(node.vis, Visibility::Public(_)) {
            let fn_str = quote::quote!(#node).to_string();

            let has_loop = fn_str.contains("for ") || fn_str.contains(".iter()") || fn_str.contains(".each(");
            let operates_on_collection = fn_str.contains("Vec") || fn_str.contains("Map");
            let checks_length = fn_str.contains("len()") || fn_str.contains("count()");

            if has_loop && operates_on_collection && !checks_length {
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG003".to_string(),
                    message: format!(
                        "Public function `{}` iterates over a dynamic collection without validating its upper length bound.",
                        node.sig.ident
                    ),
                    severity: Severity::Warning,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some("Enforce a maximum size check (e.g., `require!(vec.len() <= MAX_SIZE)`) before iterating.".to_string()),
                });
            }
        }
        syn::visit::visit_impl_item_fn(self, node);
    }
}