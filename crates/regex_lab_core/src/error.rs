use thiserror::Error;

/// Stable error type for the regex lab contract.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// The regular expression pattern is syntactically invalid.
    #[error("Invalid regular expression pattern: {0}")]
    InvalidPattern(String),

    /// The pattern exceeds the maximum allowed length.
    #[error("Pattern too long: {0} bytes (max 65536)")]
    PatternTooLong(usize),

    /// The test input exceeds the maximum allowed length.
    #[error("Input too long: {0} bytes (max {1})")]
    InputTooLong(usize, usize),

    /// The replacement string references an invalid capture group.
    #[error("Replacement error: {0}")]
    ReplacementError(String),
}

/// Public constants for length limits.
pub const MAX_PATTERN_LENGTH: usize = 65536;
pub const MAX_INPUT_LENGTH: usize = 10 * 1024 * 1024; // 10 MiB

impl CoreError {
    /// Returns a stable machine error code for Web, CLI, and Agent consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidPattern(_) => "INVALID_PATTERN",
            Self::PatternTooLong(_) => "PATTERN_TOO_LONG",
            Self::InputTooLong(_, _) => "INPUT_TOO_LONG",
            Self::ReplacementError(_) => "REPLACEMENT_ERROR",
        }
    }
}
