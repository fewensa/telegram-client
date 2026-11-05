# Contributing

Thanks for helping with `telegram-client`.

## What you need

- Rust 1.89 or newer. CI uses current stable.
- A `libtdjson` shared library on the linker path. `build.rs` also searches `./lib`. The unversioned library name (`libtdjson.so` on Linux) has to exist, not only the versioned soname. This release tracks TDLib 1.8.69.
- The typed API is generated from `td_api.tl` by the `fantasy` generator in the `rtdlib` repository (`generate/`). Regenerating `src/api` and `src/listener` (and the `TdType` match in `src/observer.rs`) is required when the schema changes. Do not hand-edit those tables.

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

The crate version tracks the TDLib schema. 1.8.70 still binds TDLib 1.8.69 (`td_api.tl` at `4c6377aa7386c39217878a9a56d581922ad37ca4`) through `rtdlib` `=1.8.70` from crates.io. Depend on an exact version. A newer TDLib schema needs a new `rtdlib` release and a regenerated client, plus a row in `version.md`.

## Pull requests

- Open a pull request against `master`.
- Describe user-visible changes in `CHANGELOG.md`.
- Keep the `authors` field as it is. Do not add personal names.
- Do not commit Telegram session data, `api_id`, or `api_hash`.

## Releases

Tag `v<version>` on the commit that matches `Cargo.toml`. `.github/workflows/publish.yml` publishes that tag to crates.io. Cargo reads `CARGO_REGISTRY_TOKEN`; the workflow sets it from the Actions secret `TOKEN_CRATES_IO`.

## Security

Report vulnerabilities through GitHub Security Advisories. See [SECURITY.md](./SECURITY.md). Do not file a public issue for a security bug.
