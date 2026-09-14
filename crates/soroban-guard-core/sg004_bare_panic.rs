use syn::{visit::Visit, ExprMacro, File, ImplItemFn, Visibility};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct BarePanicRule;

impl Rule for BarePanicRule {
    fn code(&self) -> &'static str {
        "SG004"
    }

    fn name(&self) -> &'static str {
        "Bare Panic Call Detection"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = PanicVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
            in_public_fn: false,
            current_fn: String::new(),
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct PanicVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
    in_public_fn: bool,
    current_fn: String,
}

impl<'ast> Visit<'ast> for PanicVisitor {
    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        let prev_public = self.in_public_fn;
        let prev_fn = self.current_fn.clone();

        if matches!(node.vis, Visibility::Public(_)) {
            self.in_public_fn = true;
            self.current_fn = node.sig.ident.to_string();
        }

        syn::visit::visit_impl_item_fn(self, node);

        self.in_public_fn = prev_public;
        self.current_fn = prev_fn;
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        if self.in_public_fn {
            let macro_name = node.mac.path.segments.last().unwrap().ident.to_string();
            if macro_name == "panic" || macro_name == "todo" || macro_name == "unreachable" {
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG004".to_string(),
                    message: format!(
                        "Public function `{}` contains a bare `{}!` macro call.",
                        self.current_fn, macro_name
                    ),
                    severity: Severity::Critical,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some(
                        "Use `soroban_sdk::panic_with_error!(&env, ErrorCode)` to return contract errors cleanly."
                            .to_string(),
                    ),
                });
            }
        }
        syn::visit::visit_expr_macro(self, node);
    }
}