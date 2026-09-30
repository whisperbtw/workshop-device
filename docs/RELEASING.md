# Maintainer release process

1. Update the version in Cargo.toml and run Cargo to update Cargo.lock. Keep rust-toolchain.toml consistent with the documented minimum.
2. Update RELEASE-NOTES.md and versioned installer/archive examples in all three README languages. Check that the website still points to Releases/latest.
3. Run formatting, Clippy, tests, cargo-audit and a release build. Assess every advisory; do not bypass a new vulnerability finding to publish.
4. Regenerate THIRD-PARTY-NOTICES.html with cargo-about 0.9.2. Review changes against about.toml.
5. Run scripts/package.ps1. Check the installer, portable contents, file metadata and checksums. Exercise an actual permitted download and inspect the UI at Windows display scaling used for validation.
6. Commit to main, wait for Windows validation to pass, then create and push a tag matching the version, for example v1.0.1.
7. The tag workflow validates again, packages the Windows release, saves artifacts and publishes a GitHub Release if that tag does not already have one. Existing release assets are preserved.
8. Open the published release and verify installer, ZIP, standalone executable, both license notice files and SHA256SUMS.txt. The public site is deployed from main:/docs by GitHub Pages.

For manual publication, use gh release create with --verify-tag and --notes-file RELEASE-NOTES.md. Attach all top-level dist files; never upload a stage directory or runtime cache. The initial release is unsigned. A future signed release needs the maintainer's own signing certificate; do not store signing keys in source control.
