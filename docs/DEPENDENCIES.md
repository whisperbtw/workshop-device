# Dependency maintenance

The release lockfile was audited with cargo-audit 0.22.2 on 2026-09-30. The audit reported **zero known vulnerability findings**, ten unmaintained-package warnings and one yanked version. This is an advisory-database check, not proof that all dependencies are safe.

| Package | Version | Maintenance advisory |
| --- | --- | --- |
| async-std | 1.13.2 | RUSTSEC-2025-0052 |
| instant | 0.1.13 | RUSTSEC-2024-0384 |
| paste | 1.0.15 | RUSTSEC-2024-0436 |
| proc-macro-error2 | 2.0.1 | RUSTSEC-2026-0173 |
| rustls-pemfile | 2.2.0 | RUSTSEC-2025-0134 |
| rustybuzz | 0.14.1, 0.20.1 | RUSTSEC-2026-0206 |
| ttf-parser | 0.20.0, 0.21.1, 0.25.1 | RUSTSEC-2026-0192 |

The lockfile also includes yanked yoke-derive 0.8.3. These entries are primarily transitive dependencies in the current GPUI/component graph. They remain a maintenance task; the release does not claim an entirely clean dependency audit. The pinned framework versions preserve the native renderer behavior tested locally. Dependency upgrades should be handled in a focused PR with rendering, cancellation and download regression checks.

CI runs cargo-audit without ignoring vulnerability advisories. Maintenance/yanked notices remain visible in its output. Generate license notices after dependency changes with:

```powershell
cargo install cargo-about --version 0.9.2 --locked
cargo about generate --locked --fail scripts/licenses.hbs -o THIRD-PARTY-NOTICES.html
```

Review generated notices and any license-policy changes before committing. The Windows x64 dependency graph is selected in about.toml. SteamCMD is downloaded at runtime and is not included in the distributed installer.
