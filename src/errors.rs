//! Structured error types for TheShit.
//!
//! All public-facing error paths use [`TheShitError`] so that callers get
//! clear, actionable messages instead of a bare panic.

use thiserror::Error;

/// The top-level error type for the TheShit application.
#[derive(Debug, Error)]
pub enum TheShitError {
    /// Wraps any `std::io::Error` transparently.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Returned when neither the `--shell` flag nor the environment/process
    /// tree resolves to a known shell.
    #[error("Could not determine the current shell. Pass --shell <bash|zsh|fish> to set it explicitly.")]
    ShellDetectionFailed,

    /// Returned when `env::current_exe()` fails.
    #[error("Could not locate the current executable: {0}")]
    ExePathNotFound(#[source] std::io::Error),

    /// Returned when the `SH_PREV_CMD` environment variable is absent.
    #[error("SH_PREV_CMD environment variable is not set. Make sure the shell function is installed correctly (run `theshit setup`).")]
    PrevCmdNotSet,

    /// Returned when `dirs::config_dir()` returns `None`.
    #[error("Could not locate the user config directory. Your OS may not expose a standard config path.")]
    ConfigDirNotFound,

    /// Returned when a directory entry has no usable filename component.

    /// Returned when `shell_words::split` fails to parse a command string.

    /// Returned when the Python rule processing pipeline fails at the top level.
    #[error("Failed to process Python rules: {0}")]
    PythonRulesFailed(String),
}
