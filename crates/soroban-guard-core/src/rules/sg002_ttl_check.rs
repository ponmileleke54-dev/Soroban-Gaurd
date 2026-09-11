use syn::{
    visit::Visit,
    Expr,
    ExprMethodCall,
    File,
    ImplItemFn,
    ItemImpl,
    Visibility,
};
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
            in_contract_impl: false,
        };
        visitor.visit_file(ast);
        visitor.diagnostics
    }
}

struct TtlVisitor {
    diagnostics: Vec<Diagnostic>,
    file_path: String,
    in_contract_impl: bool,
}

impl<'ast> Visit<'ast> for TtlVisitor {
    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let previous = self.in_contract_impl;
        self.in_contract_impl = node
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("contractimpl"));
        syn::visit::visit_item_impl(self, node);
        self.in_contract_impl = previous;
    }

    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        if self.in_contract_impl && matches!(node.vis, Visibility::Public(_)) {
            let mut expression_visitor = StorageExpressionVisitor::default();
            expression_visitor.visit_block(&node.block);

            if expression_visitor.uses_persistent_or_instance && !expression_visitor.extends_ttl {
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

#[derive(Default)]
struct StorageExpressionVisitor {
    uses_persistent_or_instance: bool,
    extends_ttl: bool,
}

impl<'ast> Visit<'ast> for StorageExpressionVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method = node.method.to_string();
        if method == "persistent" || method == "instance" {
            if matches!(node.receiver.as_ref(), Expr::MethodCall(receiver) if receiver.method == "storage") {
                self.uses_persistent_or_instance = true;
            }
        } else if method == "extend_ttl" {
            self.extends_ttl = true;
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diagnostics_for(source: &str) -> Vec<Diagnostic> {
        let ast = syn::parse_file(source).expect("fixture should parse");
        TtlExtensionRule.check(&ast, "fixture.rs")
    }

    #[test]
    fn flags_instance_storage_without_ttl_extension() {
        let source = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/ttl_contract.rs"
        ));
        let diagnostics = diagnostics_for(source);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("missing_instance_ttl"));
    }

    #[test]
    fn flags_persistent_storage_without_ttl_extension() {
        let diagnostics = diagnostics_for(
            "
            #[contractimpl]
            impl Contract {
                pub fn missing_persistent_ttl(env: Env) {
                    env.storage().persistent().set(&key, &value);
                }
            }
            ",
        );

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("missing_persistent_ttl"));
    }

    #[test]
    fn ignores_storage_with_ttl_and_non_storage_functions() {
        let diagnostics = diagnostics_for(
            "
            #[contractimpl]
            impl Contract {
                pub fn extended(env: Env) {
                    env.storage().persistent().extend_ttl(100, 100);
                    env.storage().persistent().set(&key, &value);
                }

                pub fn no_storage(value: i128) {
                    let _ = value;
                }
            }
            ",
        );

        assert!(diagnostics.is_empty());
    }
}