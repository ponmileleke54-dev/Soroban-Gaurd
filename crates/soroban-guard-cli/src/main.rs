use clap::{Parser, ValueEnum};
use soroban_guard_core::{
    GuardConfig, LinterEngine, Severity,
    rules::{
        BarePanicRule, HardcodedKeyRule, ReentrancyStateMutationRule, RequireAuthRule,
        TtlExtensionRule, UnboundedLoopRule, UncheckedArithmeticRule, UnusedReturnRule,
    },
};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Github,
}

#[derive(Parser)]
#[command(name = "soroban-guard")]
#[command(about = "Static Analysis Security Linter for Soroban Smart Contracts")]
struct Cli {
    #[arg(value_name = "PATH")]
    path: PathBuf,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(short, long, value_name = "CONFIG_PATH")]
    config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let config_path = cli.config.unwrap_or_else(|| PathBuf::from(".soroban-guard.toml"));
    let config = GuardConfig::load_from_path(&config_path);

    let mut engine = LinterEngine::with_config(config);
    engine.register_rule(Box::new(RequireAuthRule));
    engine.register_rule(Box::new(TtlExtensionRule));
    engine.register_rule(Box::new(UnboundedLoopRule));
    engine.register_rule(Box::new(BarePanicRule));
    engine.register_rule(Box::new(HardcodedKeyRule));
    engine.register_rule(Box::new(UncheckedArithmeticRule));
    engine.register_rule(Box::new(UnusedReturnRule));
    engine.register_rule(Box::new(ReentrancyStateMutationRule));

    match engine.analyze_file(&cli.path) {
        Ok(diagnostics) => {
            let output_str = match cli.format {
                OutputFormat::Json => serde_json::to_string_pretty(&diagnostics)
                    .unwrap_or_else(|_| "[]".to_string()),
                OutputFormat::Github => {
                    let mut buffer = String::new();
                    for diag in &diagnostics {
                        let level = match diag.severity {
                            Severity::Critical => "error",
                            Severity::Warning => "warning",
                            Severity::Info => "notice",
                        };
                        buffer.push_str(&format!(
                            "::{} file={},line={},col={}::[{}] {}\n",
                            level, diag.file_path, diag.line, diag.column, diag.rule_code, diag.message
                        ));
                    }
                    buffer
                }
                OutputFormat::Text => {
                    if diagnostics.is_empty() {
                        return println!("\x1b[1;32m✅ Analysis complete: Zero security vulnerabilities detected!\x1b[0m");
                    }

                    let mut buffer = String::new();
                    let mut critical_count = 0;
                    let mut warning_count = 0;
                    let mut info_count = 0;

                    for diag in &diagnostics {
                        let severity_tag = match diag.severity {
                            Severity::Critical => {
                                critical_count += 1;
                                "\x1b[1;31m🚨 [CRITICAL]\x1b[0m"
                            }
                            Severity::Warning => {
                                warning_count += 1;
                                "\x1b[1;33m⚠️  [WARNING]\x1b[0m"
                            }
                            Severity::Info => {
                                info_count += 1;
                                "\x1b[1;34mℹ️  [INFO]\x1b[0m"
                            }
                        };

                        buffer.push_str(&format!("{} Code: {}\n", severity_tag, diag.rule_code));
                        buffer.push_str(&format!("  File: {}\n", diag.file_path));
                        buffer.push_str(&format!("  Message: {}\n", diag.message));
                        if let Some(ref suggestion) = diag.suggestion {
                            buffer.push_str(&format!("  💡 Fix: {}\n\n", suggestion));
                        }
                    }

                    buffer.push_str("--------------------------------------------------\n");
                    buffer.push_str(&format!(
                        "Analysis Summary: {} Critical | {} Warnings | {} Info\n",
                        critical_count, warning_count, info_count
                    ));
                    buffer
                }
            };

            if let Some(out_path) = cli.output {
                if let Ok(mut file) = File::create(&out_path) {
                    let _ = file.write_all(output_str.as_bytes());
                    println!("Report written to {:?}", out_path);
                } else {
                    eprintln!("Failed to write report to {:?}", out_path);
                }
            } else {
                println!("{}", output_str);
            }
        }
        Err(err) => {
            eprintln!("\x1b[1;31m❌ Analysis error: {}\x1b[0m", err);
            std::process::exit(1);
        }
    }
}