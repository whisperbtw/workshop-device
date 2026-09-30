# Contributing

[English](CONTRIBUTING.md) · [Português](docs/CONTRIBUTING.pt-BR.md) · [日本語](docs/CONTRIBUTING.ja.md)

**Contributions are welcome.** You can help with bugs, translations, accessibility, tests, documentation and focused improvements to the native device interface.

## Before you start

Search existing issues and pull requests. For a large feature, open an issue describing the problem and proposed behavior before building it. Small fixes can go straight to a pull request. Reports and discussions may be written in English, Portuguese or Japanese.

Do not upload Steam credentials, personal filesystem paths, private logs, downloaded mods or copyrighted third-party content. Respect mod authors and Steam's access restrictions.

## Development workflow

1. Fork the repository and create a branch from `main`.
2. Install Rust 1.96.1+ with MSVC, C++ Build Tools and Windows SDK.
3. Make one focused change. Keep the interface native in GPUI and preserve compact control sizes, rounded transparency and physical press animation.
4. Add a regression test when behavior changes. For UI changes, include a real screenshot or short recording from Windows and list the scaling factor used.
5. Run:

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

6. Open a pull request explaining the problem, resulting behavior, validation and any remaining limitations. Link the relevant issue.

New user-visible strings belong in `src/i18n.rs`. Supply English, Portuguese and Japanese translations, or explicitly request translation help in the PR. Keep all README versions in sync when behavior changes.

## Quality and safety

Keep modules focused. Preserve strict Workshop URL validation, anonymous-only access, cancellation, subprocess cleanup and copying without overwrite. Do not add telemetry or a credential flow without a prior design discussion. Include Cargo.lock when dependencies change, review their licenses and regenerate third-party notices. Never execute user-provided shell commands.

Run `cargo audit` with cargo-audit before proposing a dependency change. Existing maintenance advisories are tracked in [docs/DEPENDENCIES.md](docs/DEPENDENCIES.md); newly introduced advisories need resolution or a documented assessment. Packaging uses `scripts/package.ps1` and Inno Setup 6.

By contributing, you agree that your contribution is distributed under the project's MIT license. Be respectful, provide reproducible reports and review code rather than people.
