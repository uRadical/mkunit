use std::fmt;
use std::path::PathBuf;
use thiserror::Error;

/// Exit codes for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
    InvalidArguments = 2,
    UnitNotFound = 3,
    PermissionDenied = 4,
    SystemdError = 5,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

/// Main error type for mkunit
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum MkunitError {
    #[error("Unit '{name}' not found\n\n  Searched:\n{}", format_paths(.searched_paths))]
    UnitNotFound {
        name: String,
        searched_paths: Vec<PathBuf>,
        hint: Option<String>,
    },

    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },

    #[error("Systemd error: {message}")]
    SystemdError { message: String },

    #[error("Template error: {0}")]
    TemplateError(#[from] handlebars::TemplateError),

    #[error("Template render error: {0}")]
    RenderError(#[from] handlebars::RenderError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Validation error: {message}")]
    ValidationError { message: String },

    #[error("Interactive mode required but disabled")]
    InteractiveModeDisabled,

    #[error("Editor error: {message}")]
    EditorError { message: String },

    #[error("User cancelled operation")]
    UserCancelled,

    #[error("{0}")]
    Other(String),
}

fn format_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|p| format!("    {}", p.display()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
#[allow(clippy::match_same_arms)]
impl MkunitError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::UnitNotFound { .. } => ExitCode::UnitNotFound,
            Self::PermissionDenied { .. } => ExitCode::PermissionDenied,
            Self::InvalidArgument { .. } => ExitCode::InvalidArguments,
            Self::SystemdError { .. } => ExitCode::SystemdError,
            Self::InteractiveModeDisabled => ExitCode::InvalidArguments,
            _ => ExitCode::GeneralError,
        }
    }

    pub fn unit_not_found(name: impl Into<String>, searched_paths: Vec<PathBuf>) -> Self {
        Self::UnitNotFound {
            name: name.into(),
            searched_paths,
            hint: None,
        }
    }

    pub fn unit_not_found_with_hint(
        name: impl Into<String>,
        searched_paths: Vec<PathBuf>,
        hint: impl Into<String>,
    ) -> Self {
        Self::UnitNotFound {
            name: name.into(),
            searched_paths,
            hint: Some(hint.into()),
        }
    }

    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::PermissionDenied {
            message: message.into(),
        }
    }

    pub fn invalid_argument(message: impl Into<String>) -> Self {
        Self::InvalidArgument {
            message: message.into(),
        }
    }

    pub fn systemd_error(message: impl Into<String>) -> Self {
        Self::SystemdError {
            message: message.into(),
        }
    }

    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    pub fn editor_error(message: impl Into<String>) -> Self {
        Self::EditorError {
            message: message.into(),
        }
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::GeneralError => write!(f, "general error"),
            Self::InvalidArguments => write!(f, "invalid arguments"),
            Self::UnitNotFound => write!(f, "unit not found"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::SystemdError => write!(f, "systemd error"),
        }
    }
}

/// Result type alias for mkunit operations
pub type Result<T> = std::result::Result<T, MkunitError>;

/// Validation warning (non-fatal)
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub message: String,
    pub suggestion: Option<String>,
}

impl ValidationWarning {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(message: impl Into<String>, suggestion: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }
}

impl fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Warning: {}", self.message)?;
        if let Some(ref suggestion) = self.suggestion {
            write!(f, "\n  Hint: {suggestion}")?;
        }
        Ok(())
    }
}
