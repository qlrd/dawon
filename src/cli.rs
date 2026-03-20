//! Command-line interface — explicit, no mystery.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "dawon",
    version,
    about = "Dawon — super mini-moulinette for 42 piscine (stricter than moulinette)",
    long_about = "\
Dawon evaluates 42-school piscine C submissions with checks that go\n\
beyond what moulinette does:\n\
\n\
  1. Norminette compliance\n\
  2. Symbol-name verification via libloading\n\
  3. Forbidden-function detection (regex + nm symbol table)\n\
  4. Compilation with -Wall -Wextra -Werror + ASAN/UBSAN\n\
  5. Valgrind --leak-check=full --show-leak-kinds=all\n\
  6. Per-function test harness (fork+pipe, byte-exact comparison)\n\
     Includes edge cases moulinette skips: INT_MIN, null byte, \\0.\n\
\n\
Use 'check' to evaluate yourself, 'friend' for peer evaluation."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Evaluate rush subjects instead of the default C00 subjects.
    #[arg(long, global = true)]
    pub rush: bool,

    /// Disable ASAN/UBSAN (faster, but misses runtime errors).
    #[arg(long, global = true)]
    pub no_sanitizers: bool,

    /// Disable valgrind check (faster).
    #[arg(long, global = true)]
    pub no_valgrind: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Evaluate your own submission.
    ///
    /// Example:
    ///   dawon check --path ~/piscine/C00
    ///   dawon check --path ~/piscine/C00 --exercise ex04
    Check {
        /// Path to the module directory (contains ex00/, ex01/, ...).
        #[arg(short, long, value_name = "DIR")]
        path: PathBuf,

        /// Module to evaluate (C00..C08). Defaults to C00.
        #[arg(short, long, default_value = "C00")]
        module: String,

        /// Run only this exercise, e.g. ex04.  Runs all if omitted.
        #[arg(short, long)]
        exercise: Option<String>,
    },

    /// Evaluate a friend's submission (peer evaluation).
    ///
    /// Examples:
    ///   dawon friend --login jdoe --module C00
    ///   dawon friend --path /home/jdoe/C00
    Friend {
        /// Friend's 42 login.  Dawon searches /home/<login>/<module>/.
        #[arg(short, long, conflicts_with = "path")]
        login: Option<String>,

        /// Direct path to your friend's module directory.
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Module to evaluate (C00..C08). Defaults to C00.
        #[arg(short, long, default_value = "C00")]
        module: String,

        /// Run only this exercise.
        #[arg(short, long)]
        exercise: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command};

    #[test]
    fn parse_check_rush_flag() {
        let cli = Cli::parse_from(["dawon", "--rush", "check", "--path", "/tmp/C00"]);

        assert!(cli.rush);
        assert!(matches!(cli.command, Command::Check { .. }));
    }

    #[test]
    fn parse_check_module_default() {
        let cli = Cli::parse_from(["dawon", "check", "--path", "/tmp/C00"]);

        match cli.command {
            Command::Check { module, .. } => assert_eq!(module, "C00"),
            _ => panic!("expected check command"),
        }
    }

    #[test]
    fn parse_friend_rush_flag() {
        let cli = Cli::parse_from(["dawon", "--rush", "friend", "--path", "/tmp/Rush00"]);

        assert!(cli.rush);
        assert!(matches!(cli.command, Command::Friend { .. }));
    }

    #[test]
    fn parse_friend_module_default() {
        let cli = Cli::parse_from(["dawon", "friend", "--path", "/tmp/C00"]);

        match cli.command {
            Command::Friend { module, .. } => assert_eq!(module, "C00"),
            _ => panic!("expected friend command"),
        }
    }
}
