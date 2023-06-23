use alloc::string::String;
use serde_json::json;

use crate::{id_from_value, Error, ErrorCode, Result};

/// A JSON-RPC request object
#[repr(C)]
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Request {
    jsonrpc: serde_json::Value,
    id: Option<serde_json::Value>,
    method: Option<serde_json::Value>,
    params: Option<serde_json::Value>,
}

impl Request {
    /// Creates a new [Request].
    pub fn new() -> Self {
        Self {
            jsonrpc: json!(String::from("2.0")),
            id: None,
            method: None,
            params: None,
        }
    }

    /// Creates a new [Request] with all [null](serde_json::Value::Null) fields.
    pub fn new_null() -> Self {
        Self {
            jsonrpc: serde_json::Value::Null,
            id: None,
            method: None,
            params: None,
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
        self.id.as_ref().map(|id| id_from_value(id).unwrap_or(0))
    }

    /// Sets the ID.
    pub fn set_id(&mut self, id: u64) {
        self.id = Some(json!(id));
    }

    /// Builder function to set ID.
    pub fn with_id(mut self, id: u64) -> Self {
        self.set_id(id);
        self
    }

    /// Gets the method.
    pub fn method(&self) -> Option<&str> {
        if let Some(m) = self.method.as_ref() {
            m.as_str()
        } else {
            None
        }
    }

    /// Sets the method.
    pub fn set_method(&mut self, method: &str) {
        self.method = Some(json!(String::from(method)));
    }

    /// Builder function to set method.
    pub fn with_method(mut self, method: &str) -> Self {
        self.set_method(method);
        self
    }

    /// Gets whether the params field is [null](serde_json::Value::Null).
    pub fn params_is_null(&self) -> bool {
        self.params.is_none()
    }

    /// Gets the [Request] parameters.
    ///
    /// Attempts to parse the parameter as the provided type, returns `Err(_)` on failure.
    pub fn params<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T> {
        if let Some(p) = self.params.as_ref() {
            serde_json::from_value::<T>(p.clone()).map_err(|err| err.into())
        } else {
            Err(Error::new()
                .with_code(ErrorCode::InvalidParams)
                .with_message("null Params field"))
        }
    }

    /// Sets the [Request] parameters.
    pub fn set_params<T: serde::Serialize>(&mut self, params: T) {
        self.params = Some(json!(params));
    }

    /// Builder function to set the [Request] parameters.
    pub fn with_params<T: serde::Serialize>(mut self, params: T) -> Self {
        self.set_params(params);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() -> Result<()> {
        let exp_request =
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"test_method\",\"params\":null}";
        let exp_params_request =
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"test_params\",\"params\":[0,1,2,3]}";

        let request = Request::new().with_id(1).with_method("test_method");

        let params_request = Request::new()
            .with_id(1)
            .with_method("test_params")
            .with_params([0, 1, 2, 3]);

        assert_eq!(serde_json::to_string(&request)?.as_str(), exp_request);
        assert_eq!(
            serde_json::to_string(&params_request)?.as_str(),
            exp_params_request
        );

        Ok(())
    }
}
