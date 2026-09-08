use syn::File;
use crate::diagnostic::Diagnostic;

pub trait Rule: Send + Sync {
    fn code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn check(&self, ast: &File, file_path: &str) -> Vec<Diagnostic>;
}