//! Error types for Transmutation

use std::path::PathBuf;

/// Result type alias for Transmutation operations
pub type Result<T> = std::result::Result<T, TransmutationError>;

/// Main error type for Transmutation
#[derive(Debug, thiserror::Error)]
pub enum TransmutationError {
    /// Unsupported file format
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    /// Conversion failed
    #[error("Conversion failed: {reason}")]
    ConversionFailed {
        /// Reason for failure
        reason: String,
        /// Source error if available
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Engine error
    #[error("Engine error ({engine}): {message}")]
    EngineError {
        /// Name of the engine
        engine: String,
        /// Error message
        message: String,
        /// Source error if available
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid options
    #[error("Invalid options: {0}")]
    InvalidOptions(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Timeout error
    #[error("Operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Cache error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),

    /// ML/ONNX Runtime error
    #[cfg(feature = "docling-ffi")]
    #[error("ML error: {0}")]
    MlError(String),
}

impl TransmutationError {
    /// Create a conversion failed error
    pub fn conversion_failed<S: Into<String>>(reason: S) -> Self {
        Self::ConversionFailed {
            reason: reason.into(),
            source: None,
        }
    }

    /// Create a conversion failed error with source
    pub fn conversion_failed_with_source<S: Into<String>, E>(reason: S, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ConversionFailed {
            reason: reason.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Create an engine error
    pub fn engine_error<S1: Into<String>, S2: Into<String>>(engine: S1, message: S2) -> Self {
        Self::EngineError {
            engine: engine.into(),
            message: message.into(),
            source: None,
        }
    }

    /// Create an engine error with source
    pub fn engine_error_with_source<S1: Into<String>, S2: Into<String>, E>(
        engine: S1,
        message: S2,
        source: E,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::EngineError {
            engine: engine.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::Timeout(_) | Self::NetworkError(_) | Self::CacheError(_)
        )
    }

    /// Check if error is related to configuration
    pub fn is_config_error(&self) -> bool {
        matches!(self, Self::ConfigError(_) | Self::InvalidOptions(_))
    }

    /// Check if error is related to file not found
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::FileNotFound(_))
    }
}

// ort::Error conversion for ML features
#[cfg(feature = "docling-ffi")]
impl From<ort::Error> for TransmutationError {
    fn from(err: ort::Error) -> Self {
        TransmutationError::MlError(err.to_string())
    }
}

impl From<zip::result::ZipError> for TransmutationError {
    fn from(err: zip::result::ZipError) -> Self {
        TransmutationError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = TransmutationError::conversion_failed("test");
        assert!(matches!(err, TransmutationError::ConversionFailed { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = TransmutationError::UnsupportedFormat("xyz".to_string());
        assert_eq!(err.to_string(), "Unsupported file format: xyz");
    }

    #[test]
    fn test_is_recoverable() {
        let err = TransmutationError::Timeout(std::time::Duration::from_secs(1));
        assert!(err.is_recoverable());

        let err = TransmutationError::UnsupportedFormat("test".to_string());
        assert!(!err.is_recoverable());
    }
}
