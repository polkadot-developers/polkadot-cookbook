/// Error types for the Polkadot Cookbook SDK library
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using CookbookError
pub type Result<T> = std::result::Result<T, CookbookError>;

/// Main error type for the Cookbook library
#[derive(Debug, Error, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "details")]
pub enum CookbookError {
    /// Git operation failed
    #[error("Git operation failed: {0}")]
    GitError(String),

    /// Invalid project configuration
    #[error("Invalid project configuration: {0}")]
    ConfigError(String),

    /// Scaffold generation failed
    #[error("Scaffold generation failed: {0}")]
    ScaffoldError(String),

    /// Test execution failed
    #[error("Test execution failed: {0}")]
    TestError(String),

    /// File system operation failed
    #[error("File system error: {message}")]
    FileSystemError {
        /// Error message
        message: String,
        /// Optional path where the error occurred
        #[serde(skip)]
        path: Option<PathBuf>,
    },

    /// Invalid input or validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Working directory validation failed
    #[error("Invalid working directory: {0}")]
    WorkingDirectoryError(String),

    /// Project already exists
    #[error("Project already exists at: {0}")]
    ProjectExistsError(String),

    /// Template not found
    #[error("Template not found: {0}")]
    TemplateNotFoundError(String),

    /// Bootstrap operation failed
    #[error("Bootstrap failed: {0}")]
    BootstrapError(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    IoError(String),

    /// Command execution error
    #[error("Command execution failed: {command} - {message}")]
    CommandError {
        /// Command that failed
        command: String,
        /// Error message
        message: String,
    },
}

impl From<std::io::Error> for CookbookError {
    fn from(err: std::io::Error) -> Self {
        CookbookError::IoError(err.to_string())
    }
}

impl From<serde_yaml::Error> for CookbookError {
    fn from(err: serde_yaml::Error) -> Self {
        CookbookError::ConfigError(err.to_string())
    }
}

impl From<serde_json::Error> for CookbookError {
    fn from(err: serde_json::Error) -> Self {
        CookbookError::ConfigError(err.to_string())
    }
}

impl From<regex::Error> for CookbookError {
    fn from(err: regex::Error) -> Self {
        CookbookError::ValidationError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_serialization() {
        let error = CookbookError::GitError("test error".to_string());
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("GitError"));
        assert!(json.contains("test error"));
    }

    #[test]
    fn test_error_deserialization() {
        let json = r#"{"type":"ValidationError","details":"invalid slug"}"#;
        let error: CookbookError = serde_json::from_str(json).unwrap();
        assert!(matches!(error, CookbookError::ValidationError(_)));
    }

    #[test]
    fn test_error_display() {
        let error = CookbookError::ConfigError("missing field".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid project configuration: missing field"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let cookbook_err: CookbookError = io_err.into();
        assert!(matches!(cookbook_err, CookbookError::IoError(_)));
    }

    #[test]
    fn test_serde_yaml_error_conversion() {
        let yaml = "invalid: [yaml";
        let result: std::result::Result<serde_yaml::Value, _> = serde_yaml::from_str(yaml);
        if let Err(err) = result {
            let cookbook_err: CookbookError = err.into();
            assert!(matches!(cookbook_err, CookbookError::ConfigError(_)));
        }
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json = "{invalid json}";
        let result: std::result::Result<serde_json::Value, _> = serde_json::from_str(json);
        if let Err(err) = result {
            let cookbook_err: CookbookError = err.into();
            assert!(matches!(cookbook_err, CookbookError::ConfigError(_)));
        }
    }

    #[test]
    fn test_regex_error_conversion() {
        let result = regex::Regex::new("[");
        if let Err(err) = result {
            let cookbook_err: CookbookError = err.into();
            assert!(matches!(cookbook_err, CookbookError::ValidationError(_)));
        }
    }

    #[test]
    fn test_command_error_display() {
        let error = CookbookError::CommandError {
            command: "npm install".to_string(),
            message: "failed to install".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("npm install"));
        assert!(display.contains("failed to install"));
    }

    #[test]
    fn test_file_system_error_display() {
        let error = CookbookError::FileSystemError {
            message: "permission denied".to_string(),
            path: Some(std::path::PathBuf::from("/some/path")),
        };
        let display = format!("{}", error);
        assert!(display.contains("permission denied"));
    }

    #[test]
    fn test_error_debug() {
        let error = CookbookError::GitError("test".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("GitError"));
    }
}
