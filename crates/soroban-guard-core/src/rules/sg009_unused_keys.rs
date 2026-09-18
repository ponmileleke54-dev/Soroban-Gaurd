use syn::visit::Visit;
use syn::{ItemEnum, ItemStruct, ExprMethodCall};
use crate::models::{Diagnostic, Severity};
use crate::rules::Rule;

pub struct Sg009UnusedKeys {
    defined_keys: Vec<(String, proc_macro2::Span)>,
    used_keys: Vec<String>,
}

impl Sg009UnusedKeys {
    pub fn new() -> Self {
        Self {
            defined_keys: Vec::new(),
            used_keys: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for Sg009UnusedKeys {
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        // Collect DataKey enum variants
        if node.ident.to_string().contains("DataKey") || node.ident.to_string().contains("StorageKey") {
            for variant in &node.variants {
                self.defined_keys.push((variant.ident.to_string(), variant.ident.span()));
            }
        }
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        // Collect DataKey struct definitions if applicable
        if node.ident.to_string().contains("DataKey") || node.ident.to_string().contains("StorageKey") {
            self.defined_keys.push((node.ident.to_string(), node.ident.span()));
        }
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        // Check method calls like env.storage().instance().get(&DataKey::Admin)
        let method = node.method.to_string();
        if matches!(method.as_str(), "get" | "has" | "set" | "remove" | "extend_ttl") {
            for arg in &node.args {
                let arg_str = quote::quote!(#arg).to_string();
                for (key_name, _) in &self.defined_keys {
                    if arg_str.contains(key_name) {
                        self.used_keys.push(key_name.clone());
                    }
                }
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

impl Rule for Sg009UnusedKeys {
    fn code(&self) -> &'static str {
        "SG009"
    }

    fn name(&self) -> &'static str {
        "Unused Storage Key Definition"
    }

    fn check(&mut self, file: &syn::File) -> Vec<Diagnostic> {
        self.visit_file(file);

        let mut diagnostics = Vec::new();

        for (key_name, span) in &self.defined_keys {
            if !self.used_keys.contains(key_name) {
                let line = span.start().line;
                diagnostics.push(Diagnostic {
                    rule_code: self.code().to_string(),
                    rule_name: self.name().to_string(),
                    severity: Severity::Warning,
                    message: format!(
                        "Storage key variant or struct `{}` is defined but never queried or mutated.",
                        key_name
                    ),
                    line,
                    remediation: format!(
                        "Remove unused key `{}` or implement storage operations using `env.storage()`.",
                        key_name
                    ),
                });
            }
        }

        diagnostics
    }
}