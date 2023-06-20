# smol-jsonrpc

Small `JSON-RPC` library for handling `Request` and `Response` messages in the `JSON-RPC` `2.0` protocol.

Makes no assumptions about transport layer, and is a default `no_std` library.

**Note**: the library currently requires the `alloc` crate. Future work may remove this requirement.

## Using `std`

To use `std`-only features, enable the `std` feature:

```toml
smol_jsonrpc = { version = "x.x", features = ["std"] }
```
