use syn::{visit::Visit, Expr, File, ImplItemFn, Stmt, Visibility};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct ReentrancyStateMutationRule;

impl Rule for ReentrancyStateMutationRule {
    fn code(&self) -> &'static str {
        "SG008"
    }

    fn name(&self) -> &'static str {
        "State Mutation After External Call (Reentrancy Risk)"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = ReentrancyVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
            current_fn: None,
            in_public_fn: false,
            seen_external_call: false,
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct ReentrancyVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
    current_fn: Option<String>,
    in_public_fn: bool,
    seen_external_call: bool,
}

impl<'ast> Visit<'ast> for ReentrancyVisitor {
    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        let was_public = self.in_public_fn;
        let prev_fn = self.current_fn.clone();
        let prev_call_state = self.seen_external_call;

        if matches!(node.vis, Visibility::Public(_)) {
            self.in_public_fn = true;
            self.current_fn = Some(node.sig.ident.to_string());
            self.seen_external_call = false;
        } else {
            self.in_public_fn = false;
        }

        syn::visit::visit_impl_item_fn(self, node);

        self.in_public_fn = was_public;
        self.current_fn = prev_fn;
        self.seen_external_call = prev_call_state;
    }

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        if self.in_public_fn {
            let stmt_str = quote::quote!(#node).to_string();

            // Detect external cross-contract invocations or client calls
            if stmt_str.contains("invoke_contract") || stmt_str.contains("call_external") || stmt_str.contains("Client") {
                self.seen_external_call = true;
            }

            // Flag storage modifications occurring AFTER an external call was detected in the same function
            if self.seen_external_call && (stmt_str.contains("storage().instance().set") || stmt_str.contains("storage().persistent().set")) {
                let fn_name = self.current_fn.as_deref().unwrap_or("unknown");
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG008".to_string(),
                    message: format!(
                        "Public function `{}` performs storage mutation after an external contract invocation.",
                        fn_name
                    ),
                    severity: Severity::Critical,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some(
                        "Adhere to the Checks-Effects-Interactions pattern: execute all storage mutations BEFORE triggering external calls.".to_string()
                    ),
                });
            }
        }
        syn::visit::visit_stmt(self, node);
    }
}