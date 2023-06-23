//! Error types for the library

use crate::std::{self, fmt};
use alloc::{format, string::String};
use serde::{ser::SerializeStruct, Serializer};
use serde_json::json;

/// Convenience alias for the library's [Result](std::result::Result) type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for JSON-RPC specific errors.
#[repr(C)]
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Error {
    code: ErrorCode,
    message: String,
    data: serde_json::Value,
}

impl PartialEq for Error {
    fn eq(&self, oth: &Self) -> bool {
        self.code == oth.code && self.message == oth.message
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 3)?;

        s.serialize_field("code", &i32::from(self.code))?;
        s.serialize_field("message", self.message.as_str())?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}

impl Error {
    /// Creates a new [Error].
    pub fn new() -> Self {
        Self {
            code: ErrorCode::new(),
            message: String::new(),
            data: serde_json::Value::Null,
        }
    }

    /// Gets the [ErrorCode].
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Sets the [ErrorCode].
    pub fn set_code(&mut self, code: ErrorCode) {
        self.code = code;
    }

    /// Builder function to set the [ErrorCode].
    pub fn with_code(mut self, code: ErrorCode) -> Self {
        self.code = code;
        self
    }

    /// Gets the [Error] message string.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Sets the [Error] message string.
    pub fn set_message(&mut self, message: &str) {
        self.message = String::from(message);
    }

    /// Builder function to set the [Error] message string.
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = String::from(message);
        self
    }

    /// Gets the [Error] data.
    ///
    /// The data is an extra field, and may be [null](serde_json::Value::Null).
    pub fn data(&self) -> &serde_json::Value {
        &self.data
    }

    /// Sets the [Error] data.
    pub fn set_data<T: serde::Serialize>(&mut self, data: T) {
        self.data = json!(data);
    }

    /// Builder function to set the [Error] data.
    pub fn with_data<T: serde::Serialize>(mut self, data: T) -> Self {
        self.data = json!(data);
        self
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            code: ErrorCode::ParseError,
            message: format!("{err}"),
            data: serde_json::Value::Null,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        let message = self.message();
        let data = self.data();

        if data.is_null() {
            write!(f, "\"code\": {code}, \"message\": \"{message}\"")
        } else {
            write!(
                f,
                "\"code\": {code}, \"message\": \"{message}\", \"data\": {data}"
            )
        }
    }
}

/// Error codes defined by the JSON-RPC 2.0 spec: <https://www.jsonrpc.org/specification#error_object>
///
/// Non-exhaustive, additional types for server-specific codes may be defined in the future.
#[repr(i32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ErrorCode {
    /// A parsing error occurred.
    #[default]
    ParseError = -32700,
    /// An invalid request was made.
    InvalidRequest = -32600,
    /// Method is not supported by the server.
    MethodNotFound = -32601,
    /// Invalid request parameters.
    InvalidParams = -32602,
    /// Internal server error occurred.
    InternalError = -32603,
}

impl ErrorCode {
    /// Creates a new [ErrorCode].
    pub const fn new() -> Self {
        Self::ParseError
    }
}

impl From<ErrorCode> for i32 {
    fn from(err: ErrorCode) -> Self {
        err as i32
    }
}

impl From<&ErrorCode> for i32 {
    fn from(err: &ErrorCode) -> Self {
        (*err).into()
    }
}

impl From<ErrorCode> for &'static str {
    fn from(err: ErrorCode) -> Self {
        match err {
            ErrorCode::ParseError => "Parse error",
            ErrorCode::InvalidRequest => "Invalid request",
            ErrorCode::MethodNotFound => "Method not found",
            ErrorCode::InvalidParams => "Invalid params",
            ErrorCode::InternalError => "Internal error",
        }
    }
}

impl From<&ErrorCode> for &'static str {
    fn from(err: &ErrorCode) -> Self {
        (*err).into()
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", <&'static str>::from(self), i32::from(self))
    }
}
