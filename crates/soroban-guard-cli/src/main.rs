use clap::{Parser, Subcommand};
use soroban_guard_core::{
    LinterEngine,
    rules::{
        RequireAuthRule, TtlExtensionRule, UnboundedLoopRule, BarePanicRule, HardcodedKeyRule, UncheckedArithmeticRule
    },
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
            println!("\x1b[1;36m🔍 Running soroban-guard security engine on {:?}...\x1b[0m\n", path);

            let mut engine = LinterEngine::new();
            engine.register_rule(Box::new(RequireAuthRule));
            engine.register_rule(Box::new(TtlExtensionRule));
            engine.register_rule(Box::new(UnboundedLoopRule));
            engine.register_rule(Box::new(BarePanicRule));
            engine.register_rule(Box::new(HardcodedKeyRule));
            engine.register_rule(Box::new(UncheckedArithmeticRule));

            match engine.analyze_file(&path) {
                Ok(diagnostics) => {
                    if diagnostics.is_empty() {
                        println!("\x1b[1;32m✅ Analysis complete: Zero security vulnerabilities detected!\x1b[0m");
                        return;
                    }

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

                        println!("{} \x1b[1mCode:\x1b[0m {}", severity_tag, diag.rule_code);
                        println!("  \x1b[1mFile:\x1b[0m {}", diag.file_path);
                        println!("  \x1b[1mMessage:\x1b[0m {}", diag.message);
                        if let Some(ref suggestion) = diag.suggestion {
                            println!("  \x1b[1;32m💡 Fix:\x1b[0m {}\n", suggestion);
                        }
                    }

                    println!("\x1b[1;37m--------------------------------------------------\x1b[0m");
                    println!(
                        "\x1b[1mAnalysis Summary:\x1b[0m \x1b[31m{} Critical\x1b[0m | \x1b[33m{} Warnings\x1b[0m | \x1b[34m{} Info\x1b[0m",
                        critical_count, warning_count, info_count
                    );
                }
                Err(err) => {
                    eprintln!("\x1b[1;31m❌ Analysis error: {}\x1b[0m", err);
                    std::process::exit(1);
                }
            }
        }
    }
}