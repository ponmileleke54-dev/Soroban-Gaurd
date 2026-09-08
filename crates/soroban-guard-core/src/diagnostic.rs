use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub rule_code: String,
    pub message: String,
    pub severity: Severity,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}