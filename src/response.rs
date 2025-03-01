use alloc::string::String;
use serde_json::json;

use crate::{id_from_value, Error, ErrorCode, Result};

/// A JSON-RPC response object
#[repr(C)]
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Response {
    jsonrpc: serde_json::Value,
    id: serde_json::Value,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

impl Response {
    /// Creates a new [Response].
    pub fn new() -> Self {
        Self {
            jsonrpc: json!(String::from("2.0")),
            id: serde_json::Value::Null,
            error: None,
            result: None,
        }
    }

    /// Creates a new [Response] with all [null](serde_json::Value::Null) fields.
    pub fn new_null() -> Self {
        Self {
            jsonrpc: serde_json::Value::Null,
            id: serde_json::Value::Null,
            error: None,
            result: None,
        }
    }

    /// Gets the JSON-RPC identifier string.
    ///
    /// This value should always be `"2.0"`, but may also be [null](serde_json::Value::Null) or malformed.
    pub fn jsonrpc(&self) -> Option<&str> {
        self.jsonrpc.as_str()
    }

    /// Gets the ID.
    pub fn id(&self) -> Option<u64> {
        id_from_value(&self.id)
    }

    /// Sets the ID.
    pub fn set_id(&mut self, id: u64) {
        self.id = json!(id);
    }

    /// Builder function to set ID.
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = json!(id);
        self
    }

    /// Gets whether the result field is [null](serde_json::Value::Null).
    pub fn result_is_null(&self) -> bool {
        self.result.is_none()
    }

    /// Gets the [Response] result.
    ///
    /// Attempts to parse the result as the provided type, returns `Err(_)` on failure.
    pub fn result<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T> {
        if let Some(res) = self.result.as_ref() {
            serde_json::from_value::<T>(res.clone()).map_err(|err| err.into())
        } else {
            Err(Error::new()
                .with_code(ErrorCode::InvalidParams)
                .with_message("null Result field"))
        }
    }

    /// Sets the [Response] parameters.
    pub fn set_result<T: serde::Serialize>(&mut self, result: T) {
        self.result = Some(json!(result));
    }

    /// Builder function to set the [Response] parameters.
    pub fn with_result<T: serde::Serialize>(mut self, result: T) -> Self {
        self.set_result(result);
        self
    }

    /// Gets whether the error field is [null](serde_json::Value::Null).
    pub fn error_is_null(&self) -> bool {
        self.error.is_none()
    }

    /// Gets the error.
    pub fn error(&self) -> Option<Error> {
        self.error
            .as_ref()
            .map(|err| match serde_json::from_value::<Error>(err.clone()) {
                Ok(err) => err,
                Err(err) => err.into(),
            })
    }

    /// Sets the error.
    pub fn set_error(&mut self, error: Error) {
        self.error = Some(json!(error));
    }

    /// Builder function to set error.
    pub fn with_error(mut self, error: Error) -> Self {
        self.set_error(error);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() -> Result<()> {
        let exp_res_response =
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":\"test_result\",\"error\":null}";
        let exp_err_response = "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":null,\"error\":{\"code\":-32700,\"data\":null,\"message\":\"test_error\"}}";

        let res_response = Response::new().with_id(1).with_result("test_result");

        let err = Error::new().with_message("test_error");
        let err_response = Response::new().with_id(1).with_error(err);

        assert_eq!(
            serde_json::to_string(&res_response)?.as_str(),
            exp_res_response
        );
        assert_eq!(
            serde_json::to_string(&err_response)?.as_str(),
            exp_err_response
        );

        Ok(())
    }
}
