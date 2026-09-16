use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::trait_rule::Rule;
use syn::{visit::Visit, BinOp, ExprBinary, File, ImplItemFn, Visibility};

pub struct UncheckedArithmeticRule;

impl Rule for UncheckedArithmeticRule {
    fn code(&self) -> &'static str {
        "SG006"
    }

    fn name(&self) -> &'static str {
        "Unchecked Arithmetic Operation"
    }

    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic> {
        let mut visitor = MathVisitor {
            diagnostics: Vec::new(),
            file_path: file_path.to_string(),
            current_fn: None,
            in_public_fn: false,
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct MathVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
    current_fn: Option<String>,
    in_public_fn: bool,
}

impl<'ast> Visit<'ast> for MathVisitor {
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

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        if self.in_public_fn {
            let op_str = match node.op {
                BinOp::Add(_) => Some("addition (+)"),
                BinOp::Sub(_) => Some("subtraction (-)"),
                BinOp::Mul(_) => Some("multiplication (*)"),
                _ => None,
            };

            if let Some(op_name) = op_str {
                let fn_name = self.current_fn.as_deref().unwrap_or("unknown");
                self.diagnostics.push(Diagnostic {
                    rule_code: "SG006".to_string(),
                    message: format!(
                        "Public function `{}` performs raw {} arithmetic.",
                        fn_name, op_name
                    ),
                    severity: Severity::Warning,
                    file_path: self.file_path.clone(),
                    line: 1,
                    column: 1,
                    suggestion: Some(
                        "Use `checked_add`, `checked_sub`, or `checked_mul` to prevent unexpected arithmetic overflows/underflows.".to_string()
                    ),
                });
            }
        }
        syn::visit::visit_expr_binary(self, node);
    }
}
