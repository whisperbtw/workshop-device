use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Download {
    pub id: u64,
    #[serde(default)]
    pub app_id: u32,
    #[serde(default)]
    pub game: String,
    pub title: String,
    pub folder: PathBuf,
    pub bytes: u64,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub destination: String,
    pub history: Vec<Download>,
    pub language: crate::i18n::Language,
    pub auto_open_folder: bool,
}

#[derive(Clone, Copy)]
pub enum Stage {
    Lookup,
    Preparing,
    Downloading,
    Saving,
}

impl Stage {
    pub fn text(self) -> crate::i18n::Text {
        use crate::i18n::Text;
        match self {
            Self::Lookup => Text::Lookup,
            Self::Preparing => Text::Preparing,
            Self::Downloading => Text::Downloading,
            Self::Saving => Text::Saving,
        }
    }
    pub fn label(self) -> &'static str {
        crate::i18n::Language::System.text(self.text())
    }
}

#[derive(Clone, Debug)]
pub struct ItemInfo {
    pub app_id: u32,
    pub game: String,
    pub title: String,
    pub bytes: u64,
}

#[derive(Clone, Debug)]
pub struct Failure {
    pub kind: crate::i18n::Text,
    pub diagnostic: String,
}

impl Failure {
    pub fn new(kind: crate::i18n::Text, item_id: Option<u64>, app_id: Option<u32>) -> Self {
        Self {
            kind,
            diagnostic: crate::diagnostics::report(kind, item_id, app_id),
        }
    }
}

pub enum Event {
    Stage(Stage),
    Progress { done: u64, total: u64 },
    Metadata(ItemInfo),
    Complete(Download),
    Failed(Failure),
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn old_preferences_keep_destination_and_default_to_system_language() {
        let settings: Settings =
            serde_json::from_str(r#"{"destination":"D:\\mods","history":[{"id":123,"title":"Old download","folder":"D:\\mods\\123","bytes":42}]}"#).unwrap();
        assert_eq!(settings.destination, r"D:\mods");
        assert_eq!(settings.language, crate::i18n::Language::System);
        assert!(!settings.auto_open_folder);
        assert_eq!(settings.history.len(), 1);
        assert_eq!(settings.history[0].app_id, 0);
        let restored: Settings =
            serde_json::from_slice(&serde_json::to_vec(&settings).unwrap()).unwrap();
        assert_eq!(restored.destination, settings.destination);
    }
}
