#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(not(feature = "std"))]
pub(crate) use core as std;
#[cfg(feature = "std")]
pub(crate) use std;

mod error;
mod request;
mod response;

pub use error::*;
pub use request::*;
pub use response::*;

/// Converts a JSON [Value](serde_json::Value) into a JSON-RPC ID.
///
/// Returns `Some(id)` if conversion is successful, `None` otherwise.
pub fn id_from_value(val: &serde_json::Value) -> Option<u64> {
    if val.is_null() {
        None
    } else if val.is_u64() {
        val.as_u64()
    } else if val.is_i64() {
        val.as_i64().map(|id| id as u64)
    } else if val.is_f64() {
        val.as_f64().map(|id| id as u64)
    } else if val.is_string() {
        let id_str = val.as_str().unwrap_or("");

        if id_str.contains("-") {
            id_str.parse::<i64>().ok().map(|id| id as u64)
        } else if id_str.contains(".") {
            id_str.parse::<f64>().ok().map(|id| id as u64)
        } else {
            id_str.parse::<u64>().ok()
        }
    } else {
        None
    }
}
