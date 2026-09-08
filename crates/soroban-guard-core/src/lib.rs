pub mod ast;
pub mod diagnostic;
pub mod rules;

pub use ast::LinterEngine;
pub use diagnostic::{Diagnostic, Severity};