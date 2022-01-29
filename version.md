Version mapping
===


- [telegram-client](https://github.com/fewensa/telegram-client)
- [rtdlib](https://github.com/fewensa/rtdlib)
- [td](https://github.com/tdlib/td)


The version `1.3`, `1.4`, `1.5`, `1.6`, `1.7` is outdated. the reason you can read

- [A new telegram client update](https://github.com/fewensa/telegram-client/issues/29)
- [UPDATE_APP_TO_LOGIN](https://github.com/tdlib/td/issues/1758)


A fixed version is recommended, you can read [Comparison requirements](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#comparison-requirements) about the fixed version.
Because of cargo's dependency mechanism, if you don't specify a specific version, it will be automatically upgraded, but there is usually a dependency between t and a, and the new version cannot be applied.
The current dependencies are as follows:

| telegram-client    | rtdlib      |  td |
|--------------------|-------------|-----|
| =0.8.0             | =0.8.0      | [master@fa8feef](https://github.com/tdlib/td/commit/fa8feefed70d64271945e9d5fd010b957d93c8cd) |
| =0.8.1             | =0.8.1      | [master@789b9c0](https://github.com/tdlib/td/commit/789b9c0a554d779945db027fd2612909c676345f) |
| =1.8.0             | =1.8.0      | [v1.8.0](https://github.com/tdlib/td/releases/tag/v1.8.0) |


