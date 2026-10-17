# Security Policy

## Supported versions

Security fixes are released for the latest published version of
`rentman-client` on crates.io. Upgrade to the latest version before reporting.

## Reporting a vulnerability

Please do **not** open a public issue for security problems.

Report vulnerabilities privately by e-mail to
[info@nebensound.com](mailto:info@nebensound.com), with "Security: rentman-client"
in the subject.

Include the affected version, a description of the impact, and steps or code
to reproduce. Never include a real Rentman API token; use a placeholder.

You will receive an acknowledgement within a few working days. Once a fix is
released, the advisory is published with credit to the reporter unless you
prefer to stay anonymous.

## Scope

Relevant are, for example:

- the API token appearing in logs, `Debug` output, or error messages,
- requests sent to a host other than the configured base URL,
- write operations that are repeated unexpectedly (e.g. duplicate payments),
- vulnerable dependencies that affect this crate.
