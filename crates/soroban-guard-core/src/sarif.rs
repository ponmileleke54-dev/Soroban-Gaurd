use crate::diagnostic::{Diagnostic, Severity};
use serde::Serialize;

#[derive(Serialize)]
pub struct SarifLog {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Serialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Serialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Serialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    pub information_uri: String,
}

#[derive(Serialize)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
pub struct SarifMessage {
    pub text: String,
}

#[derive(Serialize)]
pub struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
pub struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

#[derive(Serialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

#[derive(Serialize)]
pub struct SarifRegion {
    #[serde(rename = "startLine")]
    pub start_line: usize,
    #[serde(rename = "startColumn")]
    pub start_column: usize,
}

impl SarifLog {
    pub fn from_diagnostics(diagnostics: &[Diagnostic]) -> Self {
        let results = diagnostics
            .iter()
            .map(|diag| SarifResult {
                rule_id: diag.rule_code.clone(),
                level: match diag.severity {
                    Severity::Critical => "error".to_string(),
                    Severity::Warning => "warning".to_string(),
                    Severity::Info => "note".to_string(),
                },
                message: SarifMessage {
                    text: diag.message.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: diag.file_path.clone(),
                        },
                        region: SarifRegion {
                            start_line: diag.line,
                            start_column: diag.column,
                        },
                    },
                }],
            })
            .collect();

        SarifLog {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/sarif-2.1/schema/sarif-schema-2.1.0.json".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "soroban-guard".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string(),
                        information_uri: "https://github.com/gamp/Soroban-Gaurd".to_string(),
                    },
                },
                results,
            }],
        }
    }
}
