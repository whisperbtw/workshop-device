use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    System,
    Portuguese,
    English,
    Japanese,
}

impl Language {
    pub const OPTIONS: [Self; 3] = [Self::Portuguese, Self::English, Self::Japanese];
    pub fn system() -> Self {
        from_lang_id(unsafe { windows::Win32::Globalization::GetUserDefaultUILanguage() })
    }
    pub fn text(self, key: Text) -> &'static str {
        let (pt, en, ja) = key.translations();
        match self {
            Self::System => Self::system().text(key),
            Self::Portuguese => pt,
            Self::English => en,
            Self::Japanese => ja,
        }
    }
    pub fn name(self, display: Self) -> &'static str {
        match self {
            Self::System => display.text(Text::SystemLanguage),
            Self::Portuguese => "Português",
            Self::English => "English",
            Self::Japanese => "日本語",
        }
    }
}

fn from_lang_id(id: u16) -> Language {
    match id & 0x3ff {
        0x16 => Language::Portuguese,
        0x11 => Language::Japanese,
        _ => Language::English,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Text {
    Link,
    Destination,
    PasteLink,
    Download,
    Stop,
    Ready,
    Cancelling,
    Cancelled,
    Saved,
    Retry,
    Lookup,
    Preparing,
    Downloading,
    Saving,
    Settings,
    Back,
    Language,
    SystemLanguage,
    AutoOpen,
    Enabled,
    Disabled,
    SaveHere,
    PreferencesSaved,
    SettingsSaveFailed,
    LinkTooLong,
    InvalidLink,
    SteamLink,
    IndividualItem,
    SingleId,
    InvalidId,
    AbsoluteFolder,
    UnavailableItem,
    MissingGame,
    Collection,
    CancelledError,
    BadPackage,
    UnsupportedLinks,
    PrepareFailed,
    AlreadyDownloading,
    StartFailed,
    ControlFailed,
    OutputFailed,
    Timeout,
    DownloadFailed,
    MissingKey,
    AccessRequired,
    CopyDiagnostic,
    DiagnosticCopied,
    Dependencies,
    DependenciesDetail,
    EmptyFolder,
    CopyFailed,
    FolderExists,
    Interrupted,
    NetworkTimeout,
    NetworkFailed,
    PermissionDenied,
    DiskFull,
    FilesFailed,
    Unexpected,
}

impl Text {
    fn translations(self) -> (&'static str, &'static str, &'static str) {
        use Text::*;
        match self {
            Link => ("LINK DO WORKSHOP", "WORKSHOP LINK", "Workshopリンク"),
            Destination => ("SALVAR EM", "SAVE TO", "保存先"),
            PasteLink => (
                "Cole um link do Steam Workshop",
                "Paste a Steam Workshop link",
                "Workshopのリンクを貼り付け",
            ),
            Download => ("BAIXAR", "DOWNLOAD", "ダウンロード"),
            Stop => ("PARAR", "STOP", "停止"),
            Ready => ("PRONTO", "READY", "準備完了"),
            Cancelling => ("CANCELANDO", "CANCELLING", "キャンセル中"),
            Cancelled => ("CANCELADO", "CANCELLED", "キャンセル済み"),
            Saved => (
                "SALVO · ABRIR PASTA",
                "SAVED · OPEN FOLDER",
                "保存完了 · フォルダーを開く",
            ),
            Retry => (
                "VERIFIQUE E TENTE NOVAMENTE",
                "CHECK AND TRY AGAIN",
                "確認して再試行してください",
            ),
            Lookup => (
                "CONSULTANDO O ITEM",
                "CHECKING THE ITEM",
                "アイテムを確認中",
            ),
            Preparing => (
                "PREPARANDO O DOWNLOAD",
                "PREPARING DOWNLOAD",
                "ダウンロードを準備中",
            ),
            Downloading => (
                "BAIXANDO PELA STEAM",
                "DOWNLOADING FROM STEAM",
                "Steamからダウンロード中",
            ),
            Saving => (
                "SALVANDO NA SUA PASTA",
                "SAVING TO YOUR FOLDER",
                "フォルダーに保存中",
            ),
            Settings => ("CONFIGURAÇÕES", "SETTINGS", "設定"),
            Back => ("VOLTAR", "BACK", "戻る"),
            Language => ("IDIOMA", "LANGUAGE", "言語"),
            SystemLanguage => ("Seguir o sistema", "Follow system", "システムに合わせる"),
            AutoOpen => (
                "Abrir pasta ao concluir",
                "Open folder when finished",
                "完了時にフォルダーを開く",
            ),
            Enabled => ("ATIVADO", "ON", "オン"),
            Disabled => ("DESATIVADO", "OFF", "オフ"),
            SaveHere => ("Salvar aqui", "Save here", "ここに保存"),
            PreferencesSaved => ("SALVO AUTOMATICAMENTE", "SAVED AUTOMATICALLY", "自動保存"),
            SettingsSaveFailed => (
                "Não foi possível salvar as preferências. Tente novamente.",
                "Could not save preferences. Try again.",
                "設定を保存できませんでした。もう一度お試しください。",
            ),
            LinkTooLong => (
                "O link é muito longo.",
                "The link is too long.",
                "リンクが長すぎます。",
            ),
            InvalidLink => (
                "Cole o link do Workshop ou um ID numérico.",
                "Paste a Workshop link or a numeric ID.",
                "Workshopのリンクまたは数字のIDを入力してください。",
            ),
            SteamLink => (
                "Use um link HTTPS de steamcommunity.com.",
                "Use an HTTPS link from steamcommunity.com.",
                "steamcommunity.comのHTTPSリンクを使用してください。",
            ),
            IndividualItem => (
                "Abra a página de um item individual no Workshop e copie o link.",
                "Open an individual item's Workshop page and copy its link.",
                "Workshopで個別のアイテムページを開き、リンクをコピーしてください。",
            ),
            SingleId => (
                "O link deve conter um único ID do item.",
                "The link must contain exactly one item ID.",
                "リンクにはアイテムのIDを1つだけ含めてください。",
            ),
            InvalidId => (
                "ID do item inválido.",
                "Invalid item ID.",
                "アイテムのIDが無効です。",
            ),
            AbsoluteFolder => (
                "Escolha uma pasta com caminho completo usando o botão ao lado.",
                "Choose a folder with a full path using the button beside it.",
                "横のボタンで保存先フォルダーを選択してください。",
            ),
            UnavailableItem => (
                "A Steam não disponibilizou esse item. Confira se o item é público e o link está correto.",
                "Steam did not provide this item. Check that the item is public and the link is correct.",
                "Steamからこのアイテムを取得できません。アイテムが公開されているか、リンクが正しいか確認してください。",
            ),
            MissingGame => (
                "Não foi possível identificar o jogo deste item.",
                "Could not identify this item’s game.",
                "このアイテムのゲームを特定できませんでした。",
            ),
            Collection => (
                "Esse link é de uma coleção. Cole o link de um item individual.",
                "This is a collection link. Paste an individual item's link.",
                "これはコレクションのリンクです。個別のアイテムのリンクを入力してください。",
            ),
            CancelledError => (
                "Download cancelado.",
                "Download cancelled.",
                "ダウンロードをキャンセルしました。",
            ),
            BadPackage => (
                "Pacote SteamCMD maior que o esperado. Tente novamente.",
                "The SteamCMD package is larger than expected. Try again.",
                "SteamCMDのパッケージが想定より大きすぎます。再試行してください。",
            ),
            UnsupportedLinks => (
                "O item contém um atalho ou ponto de redirecionamento não suportado.",
                "The item contains an unsupported link or reparse point.",
                "このアイテムには未対応のリンクまたは再解析ポイントが含まれています。",
            ),
            PrepareFailed => (
                "Não foi possível preparar o download. Tente novamente.",
                "Could not prepare the download. Try again.",
                "ダウンロードを準備できませんでした。再試行してください。",
            ),
            AlreadyDownloading => (
                "Já existe um download em outra janela. Aguarde ou cancele antes de tentar novamente.",
                "Another window is downloading. Wait or cancel it before trying again.",
                "別のウィンドウでダウンロード中です。完了を待つかキャンセルしてください。",
            ),
            StartFailed => (
                "Não foi possível iniciar o downloader. Tente novamente.",
                "Could not start the downloader. Try again.",
                "ダウンローダーを起動できませんでした。再試行してください。",
            ),
            ControlFailed => (
                "Não foi possível controlar o downloader. Tente novamente.",
                "Could not manage the downloader process. Try again.",
                "ダウンローダーのプロセスを制御できませんでした。再試行してください。",
            ),
            OutputFailed => (
                "Saída do downloader indisponível. Tente novamente.",
                "The downloader output is unavailable. Try again.",
                "ダウンローダーの出力を取得できません。再試行してください。",
            ),
            Timeout => (
                "O download excedeu 30 minutos. Confira sua conexão e tente novamente.",
                "The download exceeded 30 minutes. Check your connection and try again.",
                "ダウンロードが30分を超えました。接続を確認して再試行してください。",
            ),
            DownloadFailed => (
                "O SteamCMD não concluiu o download. A causa não foi informada. Copie o diagnóstico e tente novamente.",
                "SteamCMD could not finish the download. No specific cause was reported. Copy the diagnostic and try again.",
                "SteamCMD がダウンロードを完了できませんでした。原因は不明です。診断情報をコピーして再試行してください。",
            ),
            MissingKey => (
                "A Steam não forneceu a chave para acessar estes arquivos no modo anônimo. Uma conta com acesso ao jogo pode ser necessária.",
                "Steam did not provide the key to access these files anonymously. An account with access to the game may be required.",
                "匿名アクセス用のファイル鍵を Steam から取得できませんでした。ゲームへのアクセス権を持つアカウントが必要な場合があります。",
            ),
            AccessRequired => (
                "A Steam negou acesso ao item. Ele pode exigir login ou acesso ao jogo pela conta; este app usa acesso anônimo.",
                "Steam denied access to the item. It may require login or game access through an account; this app uses anonymous access.",
                "Steam がアイテムへのアクセスを拒否しました。ログインやゲームへのアクセス権が必要な場合があります。このアプリは匿名アクセスを使用します。",
            ),
            CopyDiagnostic => ("Copiar diagnóstico", "Copy diagnostic", "診断情報をコピー"),
            DiagnosticCopied => (
                "Diagnóstico copiado",
                "Diagnostic copied",
                "診断情報をコピーしました",
            ),
            Dependencies => (
                "Dependências não incluídas ↗",
                "Dependencies not included ↗",
                "依存アイテムは含まれません ↗",
            ),
            DependenciesDetail => (
                "Este download não inclui outros itens exigidos. Abra a página no Workshop e confira as dependências e instruções do autor.",
                "This download does not include required items. Open the Workshop page to check dependencies and the author's instructions.",
                "必要な他のアイテムはダウンロードしません。Workshop ページで依存アイテムと作者の説明を確認してください。",
            ),
            EmptyFolder => (
                "A Steam devolveu uma pasta vazia.",
                "Steam returned an empty folder.",
                "Steamから空のフォルダーが返されました。",
            ),
            CopyFailed => (
                "A cópia não terminou. Confira o espaço e a permissão da pasta e tente novamente.",
                "The copy did not finish. Check free space and folder permissions, then try again.",
                "コピーが完了しませんでした。空き容量とフォルダーのアクセス権を確認してください。",
            ),
            FolderExists => (
                "A pasta deste download já existe. Tente novamente.",
                "This download folder already exists. Try again.",
                "このダウンロードのフォルダーは既に存在します。再試行してください。",
            ),
            Interrupted => (
                "O download foi interrompido. Tente novamente.",
                "The download was interrupted. Try again.",
                "ダウンロードが中断されました。再試行してください。",
            ),
            NetworkTimeout => (
                "A Steam demorou para responder. Confira sua conexão e tente novamente.",
                "Steam took too long to respond. Check your connection and try again.",
                "Steamの応答がタイムアウトしました。接続を確認して再試行してください。",
            ),
            NetworkFailed => (
                "Não foi possível falar com a Steam. Confira sua conexão e tente novamente.",
                "Could not connect to Steam. Check your connection and try again.",
                "Steamに接続できませんでした。接続を確認して再試行してください。",
            ),
            PermissionDenied => (
                "Sem permissão para acessar os arquivos. Escolha uma pasta onde você possa salvar.",
                "Permission denied. Choose a folder you can write to.",
                "ファイルへのアクセス権がありません。書き込み可能なフォルダーを選んでください。",
            ),
            DiskFull => (
                "O disco está cheio. Libere espaço ou escolha outra pasta.",
                "The disk is full. Free up space or choose another folder.",
                "ディスクがいっぱいです。空き容量を増やすか、別のフォルダーを選んでください。",
            ),
            FilesFailed => (
                "Não foi possível acessar os arquivos. Confira a pasta escolhida e tente novamente.",
                "Could not access the files. Check the selected folder and try again.",
                "ファイルにアクセスできませんでした。選択したフォルダーを確認してください。",
            ),
            Unexpected => (
                "Não foi possível concluir o download. Tente novamente.",
                "Could not complete the download. Try again.",
                "ダウンロードを完了できませんでした。再試行してください。",
            ),
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Language::Portuguese.text(*self))
    }
}
impl std::error::Error for Text {}

pub fn error_key(error: &anyhow::Error) -> Text {
    if let Some(network) = error.downcast_ref::<reqwest::Error>() {
        return if network.is_timeout() {
            Text::NetworkTimeout
        } else {
            Text::NetworkFailed
        };
    }
    if let Some(io) = error.downcast_ref::<std::io::Error>() {
        return match io.kind() {
            std::io::ErrorKind::PermissionDenied => Text::PermissionDenied,
            std::io::ErrorKind::StorageFull => Text::DiskFull,
            _ => Text::FilesFailed,
        };
    }
    error
        .downcast_ref::<Text>()
        .copied()
        .unwrap_or(Text::Unexpected)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recognizes_system_languages_and_falls_back_to_english() {
        assert_eq!(from_lang_id(0x0416), Language::Portuguese);
        assert_eq!(from_lang_id(0x0816), Language::Portuguese);
        assert_eq!(from_lang_id(0x0409), Language::English);
        assert_eq!(from_lang_id(0x0411), Language::Japanese);
        assert_eq!(from_lang_id(0x040c), Language::English);
    }
    #[test]
    fn error_codes_survive_context_and_translate_at_display_time() {
        let error = anyhow::Error::new(Text::Collection).context(Text::PrepareFailed);
        assert_eq!(error_key(&error), Text::PrepareFailed);
        assert_eq!(
            Language::Japanese.text(Text::MissingGame),
            "このアイテムのゲームを特定できませんでした。"
        );
    }
}
