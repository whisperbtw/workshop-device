<p align="center"><img src="assets/app-icon.png" width="180" alt="Workshop Device artwork"></p>

# Workshop Device

**Download Steam Workshop items to a folder you choose.** Paste a link to a mod, map or other Workshop item, choose where to save it and press **Download**. Workshop Device detects the game automatically and saves the item’s files in a separate folder. It is a native Windows app; you do not need to enter SteamCMD commands.

**English** · [Português](docs/README.pt-BR.md) · [日本語](docs/README.ja.md)

[Download the latest release](https://github.com/whisperbtw/workshop-device/releases/latest) · [Project website](https://whisperbtw.github.io/workshop-device/) · [Contribute](CONTRIBUTING.md) · [Report an issue](https://github.com/whisperbtw/workshop-device/issues)

The image above is application artwork, not a screenshot.

## Install

Requires Windows 10/11 x64, internet access and a graphics driver compatible with the GPUI renderer.

- **Installer:** download `Workshop-Device-Setup-1.1.0.exe` from Releases. It installs for your Windows user, creates a Start menu shortcut, offers an optional desktop shortcut and includes an uninstaller. No administrator access is required.
- **Portable:** extract `Workshop-Device-1.1.0-windows-x64.zip` and open `Workshop-Device.exe`. Keep the included license notices with it.
- `SHA256SUMS.txt` lists the SHA-256 hashes of the release files. Compare with `Get-FileHash .\Workshop-Device-Setup-1.1.0.exe -Algorithm SHA256`.

The application is intentionally unsigned; code signing is not planned. Windows may show an unknown-publisher warning. Download only from this repository's Releases.

## Use

1. Open Workshop Device.
2. Paste an individual item’s HTTPS Workshop link into **WORKSHOP LINK**. Copy the link from the item’s Steam Workshop page. A numeric Workshop ID also works.
3. Choose the destination with the small folder button.
4. Press the large **DOWNLOAD** control. The screen shows the current stage, elapsed time and errors.
5. When finished, use **SAVED · OPEN FOLDER** to find the downloaded files.

The official SteamCMD client is downloaded and prepared automatically on first use. The initial destination is your Windows Downloads folder plus `Workshop`; you can choose any writable folder. Each completed download is saved to a new `<workshop-id>-<timestamp>` subfolder. Existing downloads are not overwritten.

Drag the top grip to move the window. The engraved **−** and **×** controls minimize and close it. The download control becomes **STOP** while working. Closing the app stops its SteamCMD process.

## What it can download

- Individual Workshop items across games that allow anonymous SteamCMD downloads. The game’s app ID comes from Steam metadata; there is no fixed game or game selector.
- Collections, malformed links, items without a valid game ID and unavailable items are rejected.
- Subscribing in Steam does not trigger a download in this app: paste the link here.
- Some Workshop items require account ownership or authentication. This release has no Steam login flow and cannot download those items.
- Downloads do not install or activate an item in the game. Use the item author’s installation instructions and check compatibility with your game version.
- The application respects the access that SteamCMD provides; it does not bypass restricted content.

See [the tested items and download results](docs/COMPATIBILITY.md). Compatibility depends on each item and Steam access.

## Files, privacy and troubleshooting

Preferences, SteamCMD, cached Workshop files, receipts and logs live in `%LOCALAPPDATA%\PZWorkshopDownloader`. This legacy cache name is retained so upgrades preserve settings and cached downloads. The destination contains the copied item files. An interrupted copy may leave a hidden-style `.partial` directory; it is not a completed download.

No Steam password or API key is requested. The app contacts the Steam metadata API and the official SteamCMD download service; SteamCMD contacts Steam's servers. There is no application analytics service.

Errors appear inside the display. For a failed download, check the item is public and its game permits anonymous SteamCMD access. For saving errors, check folder permissions and free disk space. SteamCMD's own logs are inside its cache folder.

Uninstalling removes the application and shortcuts; it preserves downloaded files and the cache/preferences. To reset the app, close it and remove its cache directory yourself.

## Settings and languages

The gear on the left switches the same display to settings. Choose **English**, **Português** or **日本語**. On first launch, the app detects your Windows UI language; unsupported languages fall back to English. Your explicit selection is saved.

**Open folder when finished** is optional and off by default. Settings save automatically. Use the back arrow or the central control to return. Opening settings does not cancel an active download.

## Build and package

Install Rust **1.96.1 or later**, the MSVC toolchain, Visual Studio C++ Build Tools and the Windows SDK. Build on Windows x64:

```powershell
git clone https://github.com/whisperbtw/workshop-device.git
cd workshop-device
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

The binary is `target\release\workshop-device.exe`. The icon and file metadata are embedded during compilation.

For installer/portable packages, install [Inno Setup 6](https://jrsoftware.org/isinfo.php), then run:

```powershell
pwsh -File scripts/package.ps1
```

Packages and checksums go to `dist\`. CI validates formatting, Clippy, tests and dependency advisories. A `v*` tag builds the installer and portable archive; its version must match Cargo.toml. A successful tag build publishes a GitHub Release.

## Project layout

| Path | Purpose |
| --- | --- |
| `src/ui.rs`, `buttons.rs`, `device.rs`, `native.rs` | GPUI interface, press animation and borderless Windows behavior |
| `src/backend.rs`, `steamcmd.rs`, `install.rs` | Validation, SteamCMD lifecycle and safe file copying |
| `src/session.rs`, `model.rs`, `i18n.rs` | Download state, preferences and translations |
| `assets/` | Embedded icons and original app artwork |
| `installer/`, `scripts/`, `.github/workflows/` | Packaging and automated validation/releases |
| `docs/` | Translated documentation and the public website |

## Contributions welcome

**This project accepts contributions.** Bug fixes, translations, accessibility improvements, tests and focused native UI improvements are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. English, Portuguese and Japanese contribution guides are available.

## License and acknowledgements

Project code and artwork are distributed under [MIT](LICENSE). Dependencies retain their own licenses; [THIRD-PARTY-NOTICES.html](THIRD-PARTY-NOTICES.html) is included in distributed packages. The icon was created with an AI image-generation tool; its prompt is documented in [assets/IMAGE.md](assets/IMAGE.md).

Built with [GPUI](https://www.gpui.rs/), [gpui-component](https://github.com/longbridge/gpui-component) and the official [SteamCMD](https://developer.valvesoftware.com/wiki/SteamCMD). SteamCMD is downloaded at runtime rather than bundled. Workshop Device is a community project and is not affiliated with Valve or any game developer. Using SteamCMD is not a claim of Valve approval; respect the Steam terms and each item’s license.
