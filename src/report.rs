//! Result types shared by every check.

/// Outcome of a single check.
#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Pass,
    Fail(Vec<String>),
    Error(String),
    Pending,
    Skipped(&'static str),
}

/// Result of one named check (norminette, compiler, …).
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub status: CheckStatus,
}

/// All check results for one exercise.
#[derive(Debug, Clone)]
pub struct SuiteResult {
    pub exercise: String,
    pub function: String,
    pub checks: Vec<CheckResult>,
}

impl CheckResult {
    pub fn pass(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Pass,
        }
    }

    pub fn fail(name: impl Into<String>, msgs: Vec<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Fail(msgs),
        }
    }

    pub fn error(name: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Error(msg.into()),
        }
    }

    pub fn skip(name: impl Into<String>, reason: &'static str) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Skipped(reason),
        }
    }

    pub fn is_pass(&self) -> bool {
        matches!(self.status, CheckStatus::Pass)
    }
}

impl SuiteResult {
    pub fn pass_count(&self) -> usize {
        self.checks.iter().filter(|c| c.is_pass()).count()
    }

    pub fn total(&self) -> usize {
        self.checks.len()
    }

    pub fn all_pass(&self) -> bool {
        self.pass_count() == self.total()
    }
}
