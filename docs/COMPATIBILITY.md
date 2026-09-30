# Workshop download validation

Tested on 2026-09-30 using the published Windows x64 **Workshop Device 1.1.0** executable and anonymous SteamCMD access.

These are real downloads through the application's `--download` entry point, which uses the same validation, game detection, SteamCMD process and file-copy backend as the GUI. Each test used the item's full HTTPS Workshop URL and a separate test destination. Downloaded files were inspected, not executed or activated in a game.

## Results

| Game | Tested Workshop item | Detected app ID | Saved files | Saved bytes | Result |
| --- | --- | --- | ---: | ---: | --- |
| Don't Starve Together | [378160973](https://steamcommunity.com/sharedfiles/filedetails/?id=378160973) | `322330` | 51 | 610,208 | Downloaded; all SHA-256 hashes match |
| Terraria / tModLoader | [2619954303](https://steamcommunity.com/sharedfiles/filedetails/?id=2619954303) | `1281930` | 5 | 1,356,461 | Downloaded; all SHA-256 hashes match |
| Starbound | [3251274439](https://steamcommunity.com/sharedfiles/filedetails/?id=3251274439) | `211820` | 1 | 3,750 | Downloaded; all SHA-256 hashes match |
| Kenshi | [705119823](https://steamcommunity.com/sharedfiles/filedetails/?id=705119823) | `233860` | 3 | 690,252 | Downloaded; all SHA-256 hashes match |
| Left 4 Dead 2 | [649817121](https://steamcommunity.com/sharedfiles/filedetails/?id=649817121) | `550` | 1 | 17,195 | Downloaded; all SHA-256 hashes match |
| Garry's Mod | [557962238](https://steamcommunity.com/sharedfiles/filedetails/?id=557962238) | `4000` | 1 | 246,309 | Downloaded; all SHA-256 hashes match |
| Cities: Skylines | [2040656402](https://steamcommunity.com/sharedfiles/filedetails/?id=2040656402) | `255710` | 0 | 0 | SteamCMD returned Failure; no completed download |

A previous validation of the same release downloaded **RimWorld — Harmony**, item [2009463077](https://steamcommunity.com/sharedfiles/filedetails/?id=2009463077), app ID `294100`: 14 files, 5,729,526 bytes, with all copied SHA-256 hashes matching.

## What was checked

- The game was detected from Steam metadata without entering or selecting an app ID.
- Each successful operation produced a new `<workshop-id>-<timestamp>` folder under the requested destination.
- Every copied file had the same SHA-256 hash as the SteamCMD source, with matching file counts and total bytes.
- SteamCMD confirmed each successful item's exact ID. It exited after each attempt; no SteamCMD process was left running at the end of the test batch.
- The Cities: Skylines attempt exited with code 1, displayed a download-denied error through the app's error handling, and produced no completed folder.

## Limits of these results

Success proves that these specific items could be downloaded at this time. It does not guarantee every Workshop item for a game, future availability, game-version compatibility or correct installation inside the game. Dependencies of an item are not downloaded automatically.

For Cities: Skylines, SteamCMD connected anonymously and reported `Download item 2040656402 failed (Failure)`. Further inspection of SteamCMD’s workshop and content logs identified the underlying error: `Failed to initialize depot 255710` / `Missing decryption key`. The anonymous session did not receive the key required to read this Workshop depot. The item metadata was public and not banned. An authenticated account with the appropriate game access may resolve the missing key, but authenticated downloading was not tested. This result does not prove that every item is incompatible. The app does not bypass the failure or add a Steam login flow.

## Code signing

Windows releases are intentionally unsigned. Purchasing a signing certificate is not planned. Download from this repository's Releases and verify the published SHA-256 checksums. Windows may show an unknown-publisher warning.
