//! Error types for the PlotScript engine

use thiserror::Error;

/// Main error type for PlotScript operations
#[derive(Error, Debug)]
pub enum Error {
    /// Parse errors from the script parser
    #[error("Parse error: {0}")]
    ParseError(String),
    
    /// Runtime errors during game execution
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    /// Invalid game state
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    /// File I/O errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Invalid script format
    #[error("Invalid script: {0}")]
    InvalidScript(String),
    
    /// World model errors
    #[error("World error: {0}")]
    WorldError(String),
    
    /// Variable/scripting errors
    #[error("Script error: {0}")]
    ScriptError(String),
    
    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Permission errors
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// WebAssembly specific errors
    #[cfg(target_arch = "wasm32")]
    #[error("WASM error: {0}")]
    WasmError(String),
    
    /// Extension system errors
    #[error("Extension error: {0}")]
    ExtensionError(String),
    
    /// Unknown condition
    #[error("Unknown condition: {0}")]
    UnknownCondition(String),
    
    /// Unknown action
    #[error("Unknown action: {0}")]
    UnknownAction(String),
    
    /// Invalid arguments
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
}

/// Result type alias for PlotScript operations
pub type Result<T> = std::result::Result<T, Error>;

/// Convert pest parsing errors
impl From<pest::error::Error<crate::parser::Rule>> for Error {
    fn from(err: pest::error::Error<crate::parser::Rule>) -> Self {
        Error::ParseError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::ParseError("unexpected token".to_string());
        assert_eq!(err.to_string(), "Parse error: unexpected token");
    }
}