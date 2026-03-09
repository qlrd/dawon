//! Dawon — super mini-moulinette entry point.
//!
//! Boomer-friendly: every step is printed explicitly before it runs,
//! with a clear PASS / FAIL label and the full error output below.

use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

use dawon::config;
use dawon::eval;
use dawon::report::{CheckStatus, SuiteResult};
use dawon::subjects::{self, Subject};

mod cli;
use cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    print_banner();

    let (module_path, exercise_filter, label) = resolve_target(&args)?;

    // Load .dawon.toml if present
    let mut cfg = config::load(&module_path)?;
    if args.no_sanitizers {
        cfg.checks.no_sanitizers = true;
    }
    if args.no_valgrind {
        cfg.checks.no_valgrind = true;
    }
    if args.check_symbol {
        cfg.checks.check_symbol = true;
    }

    let subjects = subjects::all_c00();

    let to_run: Vec<&Subject> = subjects
        .iter()
        .filter(|s| {
            exercise_filter
                .as_deref()
                .map(|f| s.exercise == f)
                .unwrap_or(true)
        })
        .collect();

    if to_run.is_empty() {
        eprintln!(
            "{} no subjects found{}",
            "error:".red().bold(),
            exercise_filter
                .as_deref()
                .map(|f| format!(" for exercise '{f}'"))
                .unwrap_or_default()
        );
        std::process::exit(2);
    }

    println!("  Evaluating: {}  {}", label.bold(), module_path.display());
    println!();

    let mut all_pass = true;
    let mut suite_results: Vec<SuiteResult> = Vec::new();

    for subject in &to_run {
        let exercise_dir = module_path.join(subject.exercise);
        if !exercise_dir.is_dir() {
            print_missing_dir(subject);
            all_pass = false;
            continue;
        }

        let result = run_subject(subject, &exercise_dir, &cfg);
        if !result.all_pass() {
            all_pass = false;
        }
        suite_results.push(result);
    }

    print_grand_summary(&suite_results);
    std::process::exit(if all_pass { 0 } else { 1 });
}

// ── target resolution ──────────────────────────────────────────────

fn resolve_target(args: &Cli) -> anyhow::Result<(PathBuf, Option<String>, String)> {
    match &args.command {
        Command::Check { path, exercise } => {
            Ok((path.clone(), exercise.clone(), "myself".to_string()))
        }
        Command::Friend {
            login,
            path,
            module,
            exercise,
        } => {
            let resolved = if let Some(p) = path {
                p.clone()
            } else if let Some(login) = login {
                let m = module.as_deref().unwrap_or("C00");
                eval::find_friend_path(login, m).ok_or_else(|| {
                    anyhow::anyhow!("cannot find {login}/{m} — use --path to specify directly")
                })?
            } else {
                anyhow::bail!("friend: provide --login or --path");
            };
            let label = login
                .as_deref()
                .map(|l| format!("friend:{l}"))
                .unwrap_or_else(|| "friend".to_string());
            Ok((resolved, exercise.clone(), label))
        }
    }
}

// ── per-exercise runner + printer ─────────────────────────────────

fn run_subject(
    subject: &Subject,
    exercise_dir: &std::path::Path,
    cfg: &dawon::config::Config,
) -> SuiteResult {
    let sep = "─".repeat(60);
    println!(
        "{sep}\n  {} — {}\n  {}\n{sep}",
        subject.exercise.bold(),
        subject.function.cyan(),
        subject.description.dimmed()
    );

    let result = eval::run(subject, exercise_dir, cfg);

    let total = result.total();
    for (i, check) in result.checks.iter().enumerate() {
        let step = format!("[{}/{}]", i + 1, total);
        let name = format!("{:<42}", check.name);
        match &check.status {
            CheckStatus::Pass => {
                println!("  {} {}  {}", step.dimmed(), name, "PASS".green().bold());
            }
            CheckStatus::Fail(msgs) => {
                println!("  {} {}  {}", step.dimmed(), name, "FAIL".red().bold());
                for msg in msgs {
                    println!("         {}", msg.red());
                }
            }
            CheckStatus::Error(msg) => {
                println!("  {} {}  {}", step.dimmed(), name, "ERROR".yellow().bold());
                println!("         {}", msg.yellow());
            }
            CheckStatus::Skipped(reason) => {
                println!(
                    "  {} {}  {} ({})",
                    step.dimmed(),
                    name,
                    "SKIP".dimmed(),
                    reason
                );
            }
            CheckStatus::Pending => {
                println!("  {} {}  {}", step.dimmed(), name, "PENDING".dimmed());
            }
        }
    }

    let pass = result.pass_count();
    let total = result.total();
    let summary = format!("{pass}/{total} passed");
    if result.all_pass() {
        println!("\n  Summary: {}", summary.green().bold());
    } else {
        println!("\n  Summary: {}", summary.red().bold());
    }
    println!();

    result
}

fn print_missing_dir(subject: &Subject) {
    println!(
        "  {} — {} {}",
        subject.exercise.bold(),
        "directory not found".red(),
        format!("(expected ./{}/)", subject.exercise).dimmed()
    );
    println!();
}

// ── grand summary ──────────────────────────────────────────────────

fn print_grand_summary(results: &[SuiteResult]) {
    let sep = "═".repeat(60);
    println!("{sep}");
    println!("  GRAND SUMMARY");
    println!("{sep}");

    let mut grand_pass = 0usize;
    let mut grand_total = 0usize;

    for r in results {
        let p = r.pass_count();
        let t = r.total();
        grand_pass += p;
        grand_total += t;
        let label = if r.all_pass() {
            format!("{p}/{t}").green().bold()
        } else {
            format!("{p}/{t}").red().bold()
        };
        println!(
            "  {} {}   {}",
            r.exercise.bold(),
            format!("({})", r.function).dimmed(),
            label
        );
    }

    println!("{sep}");
    let total_label = if grand_pass == grand_total {
        format!("{grand_pass}/{grand_total} checks passed")
            .green()
            .bold()
    } else {
        format!("{grand_pass}/{grand_total} checks passed")
            .red()
            .bold()
    };
    println!("  {total_label}");
    println!("{sep}");
}

// ── banner ─────────────────────────────────────────────────────────

fn print_banner() {
    let sep = "═".repeat(60);
    println!("{sep}");
    println!(
        "  {}  {}",
        "DAWON".bold().yellow(),
        "super mini-moulinette".dimmed()
    );
    println!(
        "  {}",
        "Tiger of Mahadurga · Stricter than moulinette".dimmed()
    );
    println!("{sep}");
    println!();
}
