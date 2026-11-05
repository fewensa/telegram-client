# Changelog

All notable changes to this crate are documented in this file.

The format is based on Keep a Changelog, and this project uses semantic versioning.
The `1.8.x` version tracks the TDLib release it was generated from. 1.8.69 and 1.8.70 are the TDLib 1.8.69 schema. 1.8.70 depends on `rtdlib` `=1.8.70`.

## 1.8.70 - 2026-11-05

Maintenance release. The typed API is unchanged and still comes from TDLib 1.8.69 `td_api.tl` at `4c6377aa7386c39217878a9a56d581922ad37ca4`, through `rtdlib` 1.8.70. Publish `rtdlib` 1.8.70 to crates.io before this crate. 1.8.69 is already published.

### Changed

- Depend on `rtdlib` `=1.8.70` (`features = ["sys"]`). Same TDLib 1.8.69 schema lineage as 1.8.69.
- CI and the publish workflow use `actions/checkout` v7 (#39).
- Dev-dependency `toml` moved from 0.8.23 to the 1.x line (#40). The floor is now 1.2.0 and the lockfile resolves 1.2.0+spec-1.1.0 (#43). Examples still use `from_str`, `Value`, `as_table`, and `as_integer`. That graph drops `toml_edit` 0.22.27 and `toml_write` 0.1.2, adds `toml_parser` 1.1.5+spec-1.1.0 and `toml_writer` 1.2.0+spec-1.1.0, and updates `serde_spanned` 0.6.9 to 1.1.2, `toml_datetime` 0.6.11 to 1.1.2+spec-1.1.0, and `winnow` 0.7.15 to 1.0.5.
- Direct crate floors now name the newest releases that already fit Rust 1.89 and the previous ranges (#43). Those versions were already locked in 1.8.69: `log` 0.4.35, `regex` 1.13.1, `futures` 0.3.34, `colored` 3.1.1, `hostname` 0.4.2, `simple_logger` 5.2.0, `toolkit` 0.1.1, `tokio` 1.54.1.
- Other lockfile updates from that refresh: `bitflags` 2.11.1 to 2.13.2, `cc` 1.2.62 to 1.11.0, `find-msvc-tools` 0.1.9 to 0.1.15, `rustversion` 1.0.22 to 1.0.23, `shlex` 1.3.0 to 2.0.1, and `syn` 2.0.117 to 2.0.119. `syn` 3.0.9 is unchanged.

## 1.8.69 - 2026-11-04

Breaking schema update. Not published yet. `rtdlib` 1.8.69 has to be on crates.io before this crate can be published. 1.8.2 was prepared on this branch and was not published; its maintenance changes ship here.

### Breaking

- Depend on `rtdlib` `=1.8.69` (`features = ["sys"]`). The typed API is regenerated from TDLib 1.8.69 `td_api.tl` at `4c6377aa7386c39217878a9a56d581922ad37ca4`.
- `TdlibParameters` is removed. Build `SetTdlibParameters` directly. It gains `database_encryption_key` and `files_directory`. `enable_storage_optimizer` and `ignore_file_names` are gone. `authorizationStateWaitEncryptionKey` and `checkDatabaseEncryptionKey` are gone; the database key is set with `setTdlibParameters`. `setDatabaseEncryptionKey` only changes an existing key.
- Renames: `ChatFilter` is `ChatFolder`, `WebPage` is `LinkPreview`, `MessageForwardOrigin` is `MessageOrigin`.
- Fifteen TDLib modules are gone: `chat_filter`, `chat_filter_info`, `chat_nearby`, `chat_report_reason`, `chat_statistics_message_interaction_info`, `chats_nearby`, `file_part`, `message_forward_origin`, `payment_form_theme`, `payments_provider_stripe`, `proxies`, `recommended_chat_filter`, `recommended_chat_filters`, `tdlib_parameters`, `web_page`. Listeners and async methods for those types are gone with them.
- Minimum supported Rust is 1.89, matching `rtdlib`.

### Security

- Redact `api_id` and `api_hash` in debug logs for the JSON shapes TDLib emits (`"api_id": 12345` and `"api_hash": "..."`). The previous patterns did not match those shapes.
- Add a security policy and run `cargo audit` in CI.
- Commit `Cargo.lock` so audits and Dependabot see a reproducible dependency set.

### Changed

- Edition 2021. `lazy_static` is replaced with `std::sync::LazyLock`.
- Example dependencies are refreshed (`colored` 3, `hostname` 0.4, `simple_logger` 5, `toml` 0.8, `tokio` 1). Unused serde dev-dependencies are dropped.
- The receive loop locks the stop flag only for the check, so a writer is not blocked for the whole `td_json_client_receive` call.
- Updates are handled on one long-lived worker instead of a new thread per update. Async methods such as `getChat` are completed from the receive thread. TDLib keeps thread-local buffers for every thread that calls `td_json_client_send` and does not free them when that thread exits, so the old loop grew the process by roughly a megabyte of address space per call (#27). A response that fails to deserialize still finishes the waiting call with an error instead of leaving it parked.
- `TGError` reports its context through `Error::source`.
- Examples compile without `API_ID` / `API_HASH` at build time. Doctests that start the client are `no_run`.

### Fixed

- `Api::event`, `Api::rasync`, and `Api::new` open one TDLib client. `ApiBuilder::new` used to call `Tdlib::new`, and those constructors replaced that client, so TDLib logged a second `Create client`, then `Destroy Client` and `authorizationStateClosing` / `authorizationStateClosed` for the client that was thrown away. An `ApiBuilder` that is never built no longer opens a client. `build` still returns the same client on later calls. Method signatures are unchanged.
- Async methods whose result is an abstract TDLib class now return that class. `getOption` is the case callers hit: TDLib answers `{"@type":"optionValueString","value":"1.8.69"}`, and `optionValue*` lives under `OptionValue` rather than as a `TdType` variant, so `from_json::<TdType>` rejected the body. The same decode covers the other abstract results (`getAuthorizationState`, `getJsonValue`, `joinChat`, and the rest of that set). `testUseUpdate` returns `Update`, whose constructors are already `TdType` variants for listeners; a response carrying `@extra` is handed to the waiter as `TdType::Update`.

## 1.8.1 - 2022-01-29

- Last published release. Binds `rtdlib` 1.8.1 / TDLib v1.8.0.

Later TDLib schemas still need `rtdlib` and these wrappers regenerated together.
