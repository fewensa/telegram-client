# Security Policy

## Supported versions

| Version | Supported |
| --- | --- |
| 1.8.2 | Yes |
| 1.8.1 and older | No |
| 0.8.x and older snapshot line | No |

1.8.2 is a maintenance release of the 1.8.1 API. Fixes for supported releases are published as later 1.8.x versions.

## Reporting a vulnerability

Report security issues privately through [GitHub Security Advisories](https://github.com/fewensa/telegram-client/security/advisories/new).

Please do not open a public GitHub issue, pull request, or discussion for a vulnerability.

Include, when you can:

- The crate version and the `libtdjson` version you linked.
- A description of the impact and who can trigger it.
- A minimal reproducer that does not contain live `api_id`, `api_hash`, phone numbers, or session data.

The maintainer aims to acknowledge a report within 7 days. A fix, or a mitigation if a fix needs a new TDLib schema, will be coordinated through the advisory. You will be credited in the advisory unless you ask not to be.

## Secrets in logs

Debug logging redacts `api_id` and `api_hash` unless `ApiBuilder::unsafe_log(true)` is set. Do not enable unsafe logging on machines where logs are collected. Session directories produced by TDLib (`tdlib/` by default in the examples) are credentials. Do not commit them.
