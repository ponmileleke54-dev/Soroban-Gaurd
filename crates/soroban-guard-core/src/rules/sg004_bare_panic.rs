use syn::{visit::Visit, File, ImplItemFn, Visibility, Macro};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct BarePanicRule;

impl Rule for BarePanicRule {
    fn code(&self) -> &'static str {
        "SG004"
    }

    fn name(&self) -> &'static str {
        "Bare Panic Macro Usage"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = PanicVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
            current_fn: None,
            in_public_fn: false,
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct PanicVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
    current_fn: Option<String>,
    in_public_fn: bool,
}

impl<'ast> Visit<'ast> for PanicVisitor {
    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        let was_public = self.in_public_fn;
        let prev_fn = self.current_fn.clone();

        if matches!(node.vis, Visibility::Public(_)) {
            self.in_public_fn = true;
            self.current_fn = Some(node.sig.ident.to_string());
        } else {
            self.in_public_fn = false;
        }

        syn::visit::visit_impl_item_fn(self, node);

        self.in_public_fn = was_public;
        self.current_fn = prev_fn;
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        if self.in_public_fn {
            let path_str = quote::quote!(#node).to_string();
            if path_str.starts_with("panic") || path_str.starts_with("unreachable") || path_str.starts_with("todo") {
                let fn_name = self.current_fn.as_deref().unwrap_or("unknown");
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG004".to_string(),
                    message: format!(
                        "Public function `{}` contains an unhandled bare panic macro call.",
                        fn_name
                    ),
                    severity: Severity::Warning,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some(
                        "Replace bare `panic!()` with `soroban_sdk::panic_with_error!(&env, ErrorCode)` for structured contract errors.".to_string()
                    ),
                });
            }
        }
        syn::visit::visit_macro(self, node);
    }
}