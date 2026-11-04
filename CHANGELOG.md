# Changelog

All notable changes to this crate are documented in this file.

The format is based on Keep a Changelog, and this project uses semantic versioning.
Versions on the `1.8.x` line track the TDLib v1.8.0 schema exposed by `rtdlib` 1.8.1.

## 1.8.2 - 2026-11-04

Maintenance release. The typed TDLib surface is unchanged from 1.8.1.

### Security

- Redact `api_id` and `api_hash` in debug logs for the JSON shapes TDLib emits (`"api_id": 12345` and `"api_hash": "..."`). The previous patterns did not match those shapes.
- Add a security policy and run `cargo audit` in CI.
- Commit `Cargo.lock` so audits and Dependabot see a reproducible dependency set.

### Changed

- This is the maintained release line. It depends on `rtdlib` `=1.8.1` (TDLib v1.8.0 schema). The older `0.8.x` snapshot line is not carried forward.
- Edition 2021. Minimum supported Rust is 1.80.
- Replace `lazy_static` with `std::sync::LazyLock`.
- Refresh example dependencies (`colored` 3, `hostname` 0.4, `simple_logger` 5, `toml` 0.8, `tokio` 1) and drop unused `serde` dev-dependencies.
- The receive loop locks the stop flag only for the check, so a writer is not blocked for the whole `td_json_client_receive` call.
- `TGError` reports its context through `Error::source`.

### Fixed

- Clean build on current stable Rust, including clippy and rustfmt for the hand-written code.
- Examples compile without `API_ID` / `API_HASH` being present at build time. Those variables are read when the example runs.
- Doctests that start the client are `no_run`. `daemon` blocks until the process is stopped, and nothing in the example stops it.

## 1.8.1 - 2022-01-29

- Last previously published release. Binds `rtdlib` 1.8.1 / TDLib v1.8.0.

## Known limitations

Typed methods and objects stop at the TDLib v1.8.0 schema shipped by `rtdlib` 1.8.1 (January 2022). Later TDLib 1.8.x releases add JSON methods and update fields that this crate does not wrap.

A current `libtdjson` shared library can still be linked. The C JSON client ABI (`td_json_client_*`, `td_set_log_*`) is stable, and that is what `rtdlib-sys` calls. Unknown JSON from a newer library is reported through the existing receive and exception listeners; it is not turned into new Rust types.

Covering a newer TDLib schema means regenerating `rtdlib` (and the listener tables in this crate) from that schema. That work is not in this release.
