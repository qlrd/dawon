//! Crate-wide typed error and `Result` alias.

use std::path::PathBuf;

use thiserror::Error;

/// Dawon's central error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Wrapper for low-level I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Invalid `.dawon.toml` content.
    #[error("invalid .dawon.toml: {source}")]
    InvalidConfig {
        /// Parser error from `toml`.
        #[source]
        source: toml::de::Error,
    },

    /// A friend target needs either `--login` or `--path`.
    #[error("friend: provide --login or --path")]
    MissingFriendTarget,

    /// Could not resolve a friend's module path automatically.
    #[error("cannot find {login}/{module} — use --path to specify directly")]
    FriendPathNotFound {
        /// Friend login.
        login: String,

        /// Module name, e.g. `C00`.
        module: String,
    },

    /// Module path does not exist.
    #[error("module path not found: {path}")]
    MissingPath {
        /// Missing path.
        path: PathBuf,
    },
}

/// Crate-wide `Result` alias.
pub type Result<T> = std::result::Result<T, Error>;
