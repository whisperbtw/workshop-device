# Workshop Device 1.2.0

The device display now shows the detected game and item name, explains download failures more precisely, and offers a diagnostic you can copy for support.

- Distinguishes missing Steam depot keys, access restrictions, connection failures, unavailable items and file-saving errors when evidence is available. Generic SteamCMD failures are reported without inventing a cause.
- Uses only new SteamCMD log entries for the requested game, preventing old failures from contaminating the current diagnosis.
- **Copy diagnostic** includes the app version, error category and available Workshop/game IDs, without personal paths or raw logs.
- **Dependencies not included** opens the item's Workshop page to check requirements and the author's instructions.
- Game-name lookup is optional: downloads still work if the Steam Store cannot provide a name; the Steam app ID is shown instead.
- Keeps the device dimensions and physical button sizes. Existing preferences and download history are preserved.

Windows x64 installer, portable archive and standalone executable are included. The app uses anonymous SteamCMD access; login-required items and collections remain unsupported. Dependencies are not automatically downloaded. Releases are intentionally unsigned. Contributions welcome: see CONTRIBUTING.md.

## Português

O visor mostra o jogo e o item, diferencia as causas de erro confirmadas e permite copiar um diagnóstico sem caminhos pessoais. O aviso de dependências abre a página do item no Workshop. Mantém o tamanho do dispositivo e dos controles. Downloads que exigem login continuam sem suporte; dependências não são baixadas automaticamente.

## 日本語

画面にゲーム名とアイテム名を表示し、確認できた原因に応じてエラーを分類します。個人のパスを含まない診断情報をコピーできます。依存アイテムの注意から Workshop ページを開けます。機器とボタンのサイズは維持。ログイン必須のダウンロードには対応せず、依存アイテムは自動ダウンロードされません。
