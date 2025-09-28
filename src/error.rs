//! Error handling for the TypeScript-Rust-Compiler

use thiserror::Error;

/// Result type alias for the compiler
pub type Result<T> = std::result::Result<T, CompilerError>;

/// Main error type for the compiler
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at {line}:{column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Semantic error: {message}")]
    SemanticError { message: String },

    #[error("Generation error: {message}")]
    GenerationError { message: String },

    #[error("Unsupported TypeScript feature: {feature}")]
    UnsupportedFeature { feature: String },

    #[error("Internal compiler error: {message}")]
    InternalError { message: String },
}

impl CompilerError {
    /// Create a parse error
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(message: impl Into<String>) -> Self {
        Self::TypeError {
            message: message.into(),
        }
    }

    /// Create a semantic error
    pub fn semantic_error(message: impl Into<String>) -> Self {
        Self::SemanticError {
            message: message.into(),
        }
    }

    /// Create a generation error
    pub fn generation_error(message: impl Into<String>) -> Self {
        Self::GenerationError {
            message: message.into(),
        }
    }

    /// Create an unsupported feature error
    pub fn unsupported_feature(feature: impl Into<String>) -> Self {
        Self::UnsupportedFeature {
            feature: feature.into(),
        }
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }
}
