use crate::i18n::{Language, Text};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

pub fn report(kind: Text, item_id: Option<u64>, app_id: Option<u32>) -> String {
    let mut text = format!(
        "Workshop Device {}\nPlatform: Windows\nMode: anonymous SteamCMD\nError: {kind:?}\nReason: {}",
        env!("CARGO_PKG_VERSION"),
        Language::English.text(kind)
    );
    if let Some(id) = item_id {
        text.push_str(&format!("\nWorkshop item: {id}"));
    }
    if let Some(id) = app_id {
        text.push_str(&format!("\nSteam app: {id}"));
    }
    text
}

pub fn steam_reason(text: &str) -> Option<Text> {
    let text = text.to_ascii_lowercase();
    if text.contains("missing decryption key") {
        Some(Text::MissingKey)
    } else if text.contains("access denied")
        || text.contains("accessdenied")
        || text.contains("no subscription")
        || text.contains("not subscribed")
    {
        Some(Text::AccessRequired)
    } else if text.contains("connection timeout") || text.contains("connection timed out") {
        Some(Text::NetworkTimeout)
    } else if text.contains("failed to connect") || text.contains("no connection") {
        Some(Text::NetworkFailed)
    } else {
        None
    }
}

/// Inspect only newly written evidence for the requested app, never a previous failure.
pub struct SteamLog {
    path: PathBuf,
    offset: u64,
}
impl SteamLog {
    pub fn snapshot(cache: &Path) -> Self {
        let path = cache.join("logs/workshop_log.txt");
        let offset = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        Self { path, offset }
    }
    pub fn reason(&self, app_id: u32) -> Option<Text> {
        self.read_reason(app_id).ok().flatten()
    }
    fn read_reason(&self, app_id: u32) -> std::io::Result<Option<Text>> {
        let mut file = File::open(&self.path)?;
        let size = file.metadata()?.len();
        let offset = if size < self.offset { 0 } else { self.offset };
        file.seek(SeekFrom::Start(offset.max(size.saturating_sub(256 * 1024))))?;
        let mut bytes = Vec::new();
        file.take(256 * 1024).read_to_end(&mut bytes)?;
        let marker = format!("[AppID {app_id}]");
        Ok(String::from_utf8_lossy(&bytes)
            .lines()
            .rev()
            .filter(|line| line.contains(&marker))
            .find_map(steam_reason))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinguishes_access_connection_and_unknown_failures() {
        assert_eq!(
            steam_reason("Missing decryption key"),
            Some(Text::MissingKey)
        );
        assert_eq!(steam_reason("AccessDenied"), Some(Text::AccessRequired));
        assert_eq!(
            steam_reason("Connection timed out"),
            Some(Text::NetworkTimeout)
        );
        assert_eq!(steam_reason("Failed to connect"), Some(Text::NetworkFailed));
        assert_eq!(steam_reason("Download failed (Failure)"), None);
        let report = report(Text::MissingKey, Some(123), Some(255710));
        assert!(report.contains("Workshop item: 123"));
        assert!(report.contains("Steam app: 255710"));
        assert!(!report.contains("Users") && !report.contains("LOCALAPPDATA"));
    }
    #[test]
    fn ignores_old_log_entries_and_other_games() {
        let root = std::env::temp_dir().join(format!("workshop-evidence-{}", std::process::id()));
        std::fs::create_dir_all(root.join("logs")).unwrap();
        let path = root.join("logs/workshop_log.txt");
        std::fs::write(&path, "[AppID 255710] Missing decryption key\n").unwrap();
        let snapshot = SteamLog::snapshot(&root);
        assert_eq!(snapshot.reason(255710), None);
        use std::io::Write;
        let mut log = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        writeln!(log, "[AppID 4000] AccessDenied").unwrap();
        assert_eq!(snapshot.reason(255710), None);
        writeln!(log, "[AppID 255710] Missing decryption key").unwrap();
        drop(log);
        assert_eq!(snapshot.reason(255710), Some(Text::MissingKey));
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(root.join("logs")).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
