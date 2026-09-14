use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use soroban_guard_core::{
    rules::{RequireAuthRule, TtlExtensionRule, UnboundedLoopRule},
    LinterEngine, Severity,
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

        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Sarif,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path, format } => {
            let mut engine = LinterEngine::new();
            engine.register_rule(Box::new(RequireAuthRule));
            engine.register_rule(Box::new(TtlExtensionRule));
            engine.register_rule(Box::new(UnboundedLoopRule));

            match engine.analyze_file(&path) {
                Ok(diagnostics) => match format {
                    OutputFormat::Text => render_text_report(&path, &diagnostics),
                    OutputFormat::Json => {
                        let json = serde_json::to_string_pretty(&diagnostics).unwrap();
                        println!("{}", json);
                    }
                    OutputFormat::Sarif => render_sarif_report(&path, &diagnostics),
                },
                Err(err) => {
                    eprintln!("{} {}", "❌ Error:".red().bold(), err);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn render_text_report(path: &PathBuf, diagnostics: &[soroban_guard_core::Diagnostic]) {
    println!(
        "{}",
        format!("🔍 Running soroban-guard analysis on {:?}...\n", path).bold()
    );

    if diagnostics.is_empty() {
        println!("{}", "✅ Clean! No security vulnerabilities detected.".green().bold());
        return;
    }

    let mut critical_count = 0;
    let mut warning_count = 0;

    for diag in diagnostics {
        match diag.severity {
            Severity::Critical => {
                critical_count += 1;
                println!("{} Code: {}", "🚨 [CRITICAL]".red().bold(), diag.rule_code.bold());
            }
            Severity::Warning => {
                warning_count += 1;
                println!("{}  Code: {}", "⚠️  [WARNING]".yellow().bold(), diag.rule_code.bold());
            }
            Severity::Info => {
                println!("{}     Code: {}", "ℹ️  [INFO]".blue().bold(), diag.rule_code.bold());
            }
        }

        println!("  {} {}", "File:".dimmed(), diag.file_path);
        println!("  {} {}", "Message:".dimmed(), diag.message);
        if let Some(ref suggestion) = diag.suggestion {
            println!("  {} {}\n", "💡 Fix:".cyan(), suggestion);
        }
    }

    println!("{}", "--------------------------------------------------".dimmed());
    println!(
        "Summary: {} critical, {} warnings found.",
        critical_count.to_string().red().bold(),
        warning_count.to_string().yellow().bold()
    );
}

fn render_sarif_report(path: &PathBuf, diagnostics: &[soroban_guard_core::Diagnostic]) {
    let runs = serde_json::json!([{
        "tool": {
            "driver": {
                "name": "soroban-guard",
                "version": "0.1.0",
                "rules": [
                    { "id": "SG001", "shortDescription": { "text": "Missing Authorization Check" } },
                    { "id": "SG002", "shortDescription": { "text": "Missing State Storage TTL Extension" } },
                    { "id": "SG003", "shortDescription": { "text": "Unbounded Vector or Map Iteration" } }
                ]
            }
        },
        "results": diagnostics.iter().map(|d| {
            serde_json::json!({
                "ruleId": d.rule_code,
                "message": { "text": d.message },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": path.to_string_lossy() }
                    }
                }]
            })
        }).collect::<Vec<_>>()
    }]);

    let sarif = serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": runs
    });

    println!("{}", serde_json::to_string_pretty(&sarif).unwrap());
}