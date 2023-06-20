use alloc::string::String;
use serde_json::json;

use crate::{id_from_value, Result};

/// A JSON-RPC request object
#[repr(C)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Request {
    jsonrpc: serde_json::Value,
    id: serde_json::Value,
    method: serde_json::Value,
    params: serde_json::Value,
}

impl Request {
    /// Creates a new [Request].
    pub fn new() -> Self {
        Self {
            jsonrpc: json!(String::from("2.0")),
            id: serde_json::Value::Null,
            method: serde_json::Value::Null,
            params: serde_json::Value::Null,
        }
    }

    /// Creates a new [Request] with all [null](serde_json::Value::Null) fields.
    pub fn new_null() -> Self {
        Self {
            jsonrpc: serde_json::Value::Null,
            id: serde_json::Value::Null,
            method: serde_json::Value::Null,
            params: serde_json::Value::Null,
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

    /// Gets the method.
    pub fn method(&self) -> Option<&str> {
        self.method.as_str()
    }

    /// Sets the method.
    pub fn set_method(&mut self, method: &str) {
        self.method = json!(String::from(method));
    }

    /// Builder function to set method.
    pub fn with_method(mut self, method: &str) -> Self {
        self.method = json!(String::from(method));
        self
    }

    /// Gets whether the params field is [null](serde_json::Value::Null).
    pub fn params_is_null(&self) -> bool {
        self.params.is_null()
    }

    /// Gets the [Request] parameters.
    ///
    /// Attempts to parse the parameter as the provided type, returns `Err(_)` on failure.
    pub fn params<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T> {
        serde_json::from_value::<T>(self.params.clone()).map_err(|err| err.into())
    }

    /// Sets the [Request] parameters.
    pub fn set_params<T: serde::Serialize>(&mut self, params: &T) {
        self.params = json!(params);
    }

    /// Builder function to set the [Request] parameters.
    pub fn with_params<T: serde::Serialize>(mut self, params: T) -> Self {
        self.params = json!(params);
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
