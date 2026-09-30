use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const APP_ID: u64 = 108600;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Download {
    pub id: u64,
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

pub enum Event {
    Stage(Stage),
    Progress { done: u64, total: u64 },
    Metadata(String, u64),
    Complete(Download),
    Failed(crate::i18n::Text),
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn old_preferences_keep_destination_and_default_to_system_language() {
        let settings: Settings =
            serde_json::from_str(r#"{"destination":"D:\\mods","history":[]}"#).unwrap();
        assert_eq!(settings.destination, r"D:\mods");
        assert_eq!(settings.language, crate::i18n::Language::System);
        assert!(!settings.auto_open_folder);
        let restored: Settings =
            serde_json::from_slice(&serde_json::to_vec(&settings).unwrap()).unwrap();
        assert_eq!(restored.destination, settings.destination);
    }
}
