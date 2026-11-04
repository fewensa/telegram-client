# Contributing

Thanks for helping with `telegram-client`.

## What you need

- Rust 1.80 or newer to build the library. The examples and dev-dependencies currently need Rust 1.88 or newer. CI uses current stable.
- A `libtdjson` shared library on the linker path. `build.rs` also searches `./lib`. The unversioned library name (`libtdjson.so` on Linux) has to exist, not only the versioned soname.
- The typed API matches `rtdlib` 1.8.1 / TDLib v1.8.0. Linking a newer `libtdjson` is expected. Adding new TDLib methods is not a small patch: `rtdlib` has to be regenerated first, then the listener tables in this crate.

## Checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --all-targets
cargo test --all-features --doc
cargo audit
cargo publish --dry-run
```

`cargo test` builds the examples and runs library tests and doctests. It does not run the examples. The examples log into Telegram and block.

`src/api/aasync.rs`, `src/api/aevent.rs`, `src/listener/lasync.rs`, and `src/listener/levent.rs` are generated TDLib wrappers. Clippy's `type_complexity` lint is allowed there because those callback types are the public API. Run `cargo fmt --all` before submitting; rustfmt uses two-space indents, matching `.editorconfig`.

## Versioning

Patch releases on 1.8.x keep the `rtdlib` 1.8.1 schema. A new TDLib schema needs a new minor or a new major, together with a matching `rtdlib` crate, and a row in `version.md`. Depend on an exact version (`=1.8.2`). Cargo will not keep this crate aligned with TDLib on its own.

## Pull requests

- Open a pull request against `master`.
- Describe user-visible changes in `CHANGELOG.md`.
- Keep the `authors` field as it is. Do not add personal names.
- Do not commit Telegram session data, `api_id`, or `api_hash`.

## Security

Report vulnerabilities through GitHub Security Advisories. See [SECURITY.md](./SECURITY.md). Do not file a public issue for a security bug.
