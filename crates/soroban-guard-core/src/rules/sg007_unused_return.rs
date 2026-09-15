use syn::{visit::Visit, Expr, File, ImplItemFn, Stmt, Visibility};
use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;

pub struct UnusedReturnRule;

impl Rule for UnusedReturnRule {
    fn code(&self) -> &'static str {
        "SG007"
    }

    fn name(&self) -> &'static str {
        "Unused Result or Storage Return Value"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = ReturnVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
            current_fn: None,
            in_public_fn: false,
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct ReturnVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
    current_fn: Option<String>,
    in_public_fn: bool,
}

impl<'ast> Visit<'ast> for ReturnVisitor {
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

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        if self.in_public_fn {
            // In syn v2, statements ending with a semicolon match Stmt::Expr(expr, Some(_))
            if let Stmt::Expr(Expr::MethodCall(method_call), Some(_semi)) = node {
                let method_name = method_call.method.to_string();
                if method_name == "try_get" || method_name == "try_set" || method_name == "get" {
                    let fn_name = self.current_fn.as_deref().unwrap_or("unknown");
                    self.diagnostics.push(Diagnostic {
                        rule_code: "SG007".to_string(),
                        message: format!(
                            "Public function `{}` discards return value of storage operation `{}`.",
                            fn_name, method_name
                        ),
                        severity: Severity::Warning,
                        file_path: self.file_path.clone(),
                        line: 1,
                        column: 1,
                        suggestion: Some(
                            "Bind or handle the `Result`/`Option` returned by storage methods to prevent silently ignored failures.".to_string()
                        ),
                    });
                }
            }
        }
        syn::visit::visit_stmt(self, node);
    }
}