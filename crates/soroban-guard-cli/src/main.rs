use clap::{Parser, Subcommand};
use soroban_guard_core::{
    LinterEngine,
    rules::{RequireAuthRule, TtlExtensionRule, UnboundedLoopRule},
    Severity,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "soroban-guard")]
#[command(about = "Static Analysis Security Linter for Soroban Smart Contracts")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path } => {
            println!("🔍 Running soroban-guard analysis on {:?}...\n", path);

            let mut engine = LinterEngine::new();
            engine.register_rule(Box::new(RequireAuthRule));
            engine.register_rule(Box::new(TtlExtensionRule));
            engine.register_rule(Box::new(UnboundedLoopRule));

            match engine.analyze_file(&path) {
                Ok(diagnostics) => {
                    if diagnostics.is_empty() {
                        println!("✅ No security issues found!");
                        return;
                    }

                    for diag in &diagnostics {
                        let severity_label = match diag.severity {
                            Severity::Critical => "🚨 [CRITICAL]",
                            Severity::Warning  => "⚠️  [WARNING]",
                            Severity::Info     => "ℹ️  [INFO]",
                        };

                        println!("{} Code: {}", severity_label, diag.rule_code);
                        println!(" File: {}", diag.file_path);
                        println!(" Message: {}", diag.message);
                        if let Some(ref suggestion) = diag.suggestion {
                            println!(" 💡 Fix: {}\n", suggestion);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("❌ Analysis error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}