<p align="center"><img src="../assets/app-icon.png" width="180" alt="Workshop Device のアートワーク"></p>

# Workshop Device

**Project Zomboid の Workshop MOD を指定したフォルダーにダウンロードする、シンプルな Windows ネイティブアプリです。** Rust と GPUI を使用し、実機をイメージした縦長のインターフェースを備えています。

[English](../README.md) · [Português](README.pt-BR.md) · **日本語**

[最新版をダウンロード](https://github.com/whisperbtw/workshop-device/releases/latest) · [プロジェクトサイト](https://whisperbtw.github.io/workshop-device/) · [貢献ガイド](CONTRIBUTING.ja.md) · [問題を報告](https://github.com/whisperbtw/workshop-device/issues)

上の画像はアプリのアートワークであり、スクリーンショットではありません。

## インストール

Windows 10/11 x64、インターネット接続、GPUI の描画に対応したグラフィックスドライバーが必要です。

- **インストーラー:** Releases から `Workshop-Device-Setup-1.0.0.exe` をダウンロードします。現在の Windows ユーザー向けにインストールされ、スタートメニューのショートカットとアンインストーラーが作成されます。デスクトップのショートカットは任意です。管理者権限は不要です。
- **ポータブル版:** `Workshop-Device-1.0.0-windows-x64.zip` を展開し、`Workshop-Device.exe` を開きます。付属のライセンス通知も保管してください。
- `SHA256SUMS.txt` に配布ファイルの SHA-256 ハッシュがあります。`Get-FileHash .\Workshop-Device-Setup-1.0.0.exe -Algorithm SHA256` で確認できます。

初回リリースにはコード署名がありません。Windows が発行元不明の警告を表示する場合があります。このリポジトリの Releases からダウンロードしてください。

## 使い方

1. Workshop Device を開きます。
2. MOD リンク欄に個別 MOD の HTTPS Workshop リンクを貼り付けます。例: `https://steamcommunity.com/sharedfiles/filedetails/?id=2169435993`。数値の Workshop ID も使えます。
3. 小さなフォルダーボタンで保存先を選択します。
4. 大きなダウンロードボタンを押します。画面に処理段階、経過時間、エラーが表示されます。
5. 完了後、保存済みのフォルダーを開くボタンからファイルを確認します。

初回利用時に公式 SteamCMD が自動でダウンロード・準備されます。完了したダウンロードは新しい `<workshop-id>-<timestamp>` サブフォルダーに保存され、既存のダウンロードは上書きされません。

上部のグリップをドラッグするとウィンドウを移動できます。刻印風の **−** は最小化、**×** は終了です。ダウンロード中は中央ボタンが停止ボタンになります。アプリを閉じると、このアプリの SteamCMD プロセスも終了します。

## 設定と言語

左上の歯車で同じ画面が設定画面に切り替わります。**English**、**Português**、**日本語** から選べます。初回起動時は Windows の表示言語を検出し、対応していない言語の場合は英語になります。手動で選んだ言語は保存されます。

完了後にフォルダーを開く設定は任意で、初期状態では無効です。設定は自動保存されます。戻る矢印または中央ボタンで元の画面に戻れます。設定画面を開いてもダウンロードは中断されません。

## 対応範囲

- SteamCMD の匿名ダウンロードが許可されている Project Zomboid の個別アイテム（Steam app ID `108600`）に対応します。
- コレクション、他のゲーム、無効なリンク、利用できないアイテムは受け付けません。
- Steam でサブスクライブしても、このアプリのダウンロードは開始されません。リンクを貼り付けてください。
- Steam アカウントの認証や所有権が必要なアイテムもあります。このバージョンには Steam ログイン機能がなく、それらはダウンロードできません。
- MOD のゲームへのインストールや有効化は行いません。作者の手順に従い、ゲームのバージョンとの互換性を確認してください。
- SteamCMD が提供するアクセスを使用し、制限を回避する機能はありません。

## ファイル・プライバシー・トラブル対応

設定、SteamCMD、Workshop キャッシュ、保存記録、ログは `%LOCALAPPDATA%\PZWorkshopDownloader` に保存されます。指定した保存先には MOD のファイルがコピーされます。中断されたコピーには `.partial` の一時フォルダーが残る場合があります。これは完了したダウンロードではありません。

Steam パスワードや API キーは要求しません。アプリは Steam のメタデータ API と公式 SteamCMD 配布サービスに接続し、SteamCMD は Steam のサーバーに接続します。アプリ独自の利用分析サービスはありません。

エラーはアプリ内の画面に表示されます。失敗時はアイテムが公開されているか、Project Zomboid 向けか、匿名アクセスが許可されているかを確認してください。保存エラーの場合はフォルダーの権限と空き容量を確認します。SteamCMD 自身のログはキャッシュ内にあります。

アンインストールでアプリとショートカットが削除されますが、MOD、キャッシュ、設定は残ります。設定をリセットする場合はアプリを閉じてからキャッシュフォルダーを手動で削除してください。

## ビルドとパッケージ作成

Rust **1.96.1 以降**、MSVC ツールチェーン、Visual Studio C++ Build Tools、Windows SDK をインストールします。Windows x64 上で実行してください。

```powershell
git clone https://github.com/whisperbtw/workshop-device.git
cd workshop-device
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

実行ファイルは `target\release\pz-workshop-downloader.exe` です。ビルド時にアイコンとファイル情報が埋め込まれます。

[Inno Setup 6](https://jrsoftware.org/isinfo.php) をインストールして実行します。

```powershell
pwsh -File scripts/package.ps1
```

パッケージとチェックサムは `dist\` に出力されます。CI は整形、Clippy、テスト、依存関係のアドバイザリーを確認します。`v*` タグを付けるとパッケージが作成されます。タグのバージョンは Cargo.toml と一致する必要があります。タグのビルドに成功すると GitHub Release が公開されます。

## 構成

| パス | 内容 |
| --- | --- |
| `src/ui.rs`、`buttons.rs`、`device.rs`、`native.rs` | GPUI 画面、押下アニメーション、フレームなしウィンドウ |
| `src/backend.rs`、`steamcmd.rs`、`install.rs` | 入力検証、SteamCMD の管理、安全なコピー |
| `src/session.rs`、`model.rs`、`i18n.rs` | 状態、設定、翻訳 |
| `assets/` | アイコンとアートワーク |
| `installer/`、`scripts/`、`.github/workflows/` | パッケージ作成、検証、リリース |
| `docs/` | 翻訳ドキュメントと公開サイト |

## 貢献を歓迎します

**このプロジェクトは貢献を受け付けています。** 不具合修正、翻訳、アクセシビリティ、テスト、ネイティブ UI の改善を歓迎します。Pull Request を送る前に [貢献ガイド](CONTRIBUTING.ja.md) を読んでください。英語とポルトガル語のガイドもあります。

## ライセンスと謝辞

コードとアートワークは [MIT](../LICENSE) ライセンスで配布します。依存関係にはそれぞれのライセンスが適用され、配布物には [THIRD-PARTY-NOTICES.html](../THIRD-PARTY-NOTICES.html) が付属します。アイコンは画像生成 AI で作成され、プロンプトは [assets/IMAGE.md](../assets/IMAGE.md) に記載しています。

[GPUI](https://www.gpui.rs/)、[gpui-component](https://github.com/longbridge/gpui-component)、公式 [SteamCMD](https://developer.valvesoftware.com/wiki/SteamCMD) を使用しています。SteamCMD は実行時に取得され、配布パッケージには含まれません。Valve および The Indie Stone とは関係のないコミュニティプロジェクトです。
