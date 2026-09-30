use crate::i18n::Text;
use crate::model::{APP_ID, Download, Event, Settings, Stage};
use anyhow::{Context, Result, ensure};
use reqwest::blocking::Client;
use std::{
    fs::{self, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const STEAMCMD_URL: &str = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";

pub fn runtime() -> PathBuf {
    PathBuf::from(std::env::var_os("LOCALAPPDATA").unwrap_or_else(|| ".".into()))
        .join("PZWorkshopDownloader")
}

pub fn default_destination() -> PathBuf {
    use windows::Win32::{
        System::Com::CoTaskMemFree,
        UI::Shell::{FOLDERID_Downloads, KF_FLAG_DEFAULT, SHGetKnownFolderPath},
    };
    // Respect the current user's Downloads location, including redirected folders.
    let downloads = unsafe {
        SHGetKnownFolderPath(&FOLDERID_Downloads, KF_FLAG_DEFAULT, None)
            .ok()
            .and_then(|path| {
                let decoded = path.to_string().ok().map(PathBuf::from);
                CoTaskMemFree(Some(path.as_ptr().cast()));
                decoded
            })
    };
    downloads
        .or_else(|| {
            std::env::var_os("USERPROFILE").map(|home| PathBuf::from(home).join("Downloads"))
        })
        .unwrap_or_else(runtime)
        .join("ProjectZomboid")
        .join("ModsBaixados")
}

pub fn load_settings() -> Settings {
    fs::read(runtime().join("settings.json"))
        .ok()
        .and_then(|s| serde_json::from_slice(&s).ok())
        .unwrap_or_default()
}

pub fn save_settings(settings: &Settings) -> Result<()> {
    fs::create_dir_all(runtime())?;
    let temporary = runtime().join(format!("settings-{}.tmp", std::process::id()));
    fs::write(&temporary, serde_json::to_vec_pretty(settings)?)?;
    fs::rename(temporary, runtime().join("settings.json"))?;
    Ok(())
}

pub fn parse_id(input: &str) -> Result<u64> {
    let input = input.trim();
    ensure!(input.len() <= 4096, Text::LinkTooLong);
    let id = if input.bytes().all(|c| c.is_ascii_digit()) && !input.is_empty() {
        input.to_string()
    } else {
        let url = url::Url::parse(input).context(Text::InvalidLink)?;
        ensure!(
            url.scheme() == "https"
                && url.host_str() == Some("steamcommunity.com")
                && url.username().is_empty()
                && url.password().is_none()
                && url.port_or_known_default() == Some(443),
            Text::SteamLink
        );
        ensure!(
            [
                "/sharedfiles/filedetails/",
                "/sharedfiles/filedetails",
                "/workshop/filedetails/",
                "/workshop/filedetails"
            ]
            .contains(&url.path()),
            Text::IndividualMod
        );
        let ids: Vec<_> = url.query_pairs().filter(|(key, _)| key == "id").collect();
        ensure!(ids.len() == 1, Text::SingleId);
        ids[0].1.to_string()
    };
    ensure!(
        !id.is_empty() && id.bytes().all(|c| c.is_ascii_digit()),
        Text::InvalidId
    );
    let number: u64 = id.parse().context(Text::InvalidId)?;
    ensure!(number > 0, Text::InvalidId);
    Ok(number)
}

fn cancelled(cancel: &AtomicBool) -> Result<()> {
    ensure!(!cancel.load(Ordering::Relaxed), Text::CancelledError);
    Ok(())
}

fn client() -> Result<Client> {
    Ok(Client::builder()
        .user_agent("PZWorkshopDownloader/1.0")
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(90))
        .redirect(reqwest::redirect::Policy::none())
        .build()?)
}

fn metadata(client: &Client, id: u64) -> Result<(String, u64)> {
    let json: serde_json::Value = client
        .post("https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/")
        .form(&[
            ("itemcount", "1".to_string()),
            ("publishedfileids[0]", id.to_string()),
        ])
        .send()?
        .error_for_status()?
        .json()?;
    let detail = &json["response"]["publishedfiledetails"][0];
    ensure!(detail["result"].as_u64() == Some(1), Text::UnavailableItem);
    ensure!(
        detail["consumer_app_id"].as_u64() == Some(APP_ID),
        Text::WrongGame
    );
    ensure!(detail["file_type"].as_u64() != Some(2), Text::Collection);
    let size = detail["file_size"]
        .as_u64()
        .or_else(|| detail["file_size"].as_str().and_then(|n| n.parse().ok()))
        .unwrap_or(0);
    Ok((
        detail["title"]
            .as_str()
            .unwrap_or("Mod do Workshop")
            .to_string(),
        size,
    ))
}

fn bootstrap(
    client: &Client,
    steam: &Path,
    tx: &Sender<Event>,
    cancel: &AtomicBool,
) -> Result<PathBuf> {
    let exe = steam.join("steamcmd.exe");
    if exe.is_file() {
        return Ok(exe);
    }
    let _ = tx.send(Event::Stage(Stage::Preparing));
    fs::create_dir_all(steam)?;
    let mut response = client.get(STEAMCMD_URL).send()?.error_for_status()?;
    let total = response.content_length().unwrap_or(0);
    let mut data = Vec::new();
    let mut buffer = [0u8; 65536];
    loop {
        cancelled(cancel)?;
        let count = response.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        ensure!(data.len() + count <= 50 * 1024 * 1024, Text::BadPackage);
        data.extend_from_slice(&buffer[..count]);
        let _ = tx.send(Event::Progress {
            done: data.len() as u64,
            total,
        });
    }
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))?;
    let mut entry = archive.by_name("steamcmd.exe")?;
    ensure!(entry.size() <= 50 * 1024 * 1024, Text::BadPackage);
    let temporary = steam.join("steamcmd.exe.part");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temporary)?;
    std::io::copy(&mut entry, &mut file)?;
    file.sync_all()?;
    drop(file);
    fs::rename(temporary, &exe)?;
    Ok(exe)
}

fn run_download(
    id: u64,
    destination: &Path,
    tx: &Sender<Event>,
    cancel: &AtomicBool,
) -> Result<Download> {
    ensure!(destination.is_absolute(), Text::AbsoluteFolder);
    fs::create_dir_all(runtime())?;
    let lock_path = runtime().join("download.lock");
    let lock_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .context(Text::PrepareFailed)?;
    lock_file.try_lock().context(Text::AlreadyDownloading)?;
    let client = client()?;
    let _ = tx.send(Event::Stage(Stage::Lookup));
    let (title, expected_size) = metadata(&client, id)?;
    let _ = tx.send(Event::Metadata(title.clone(), expected_size));
    cancelled(cancel)?;
    let steam = runtime().join("steamcmd");
    let exe = bootstrap(&client, &steam, tx, cancel).context(Text::PrepareFailed)?;
    cancelled(cancel)?;
    let _ = tx.send(Event::Stage(Stage::Downloading));
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let log_path = runtime().join(format!("download-{id}-{stamp}.log"));
    crate::steamcmd::download(&exe, &steam, id, &log_path, cancel)?;
    let source = steam.join(format!("steamapps/workshop/content/{APP_ID}/{id}"));
    ensure!(fs::read_dir(&source)?.next().is_some(), Text::EmptyFolder);
    cancelled(cancel)?;
    let _ = tx.send(Event::Stage(Stage::Saving));
    fs::create_dir_all(destination)?;
    let destination = destination.canonicalize()?;
    let folder = destination.join(format!("{id}-{stamp}"));
    let staging = destination.join(format!(".{id}-{stamp}.partial"));
    let bytes = crate::install::copy(&source, &staging, tx, cancel).context(Text::CopyFailed)?;
    let result = Download {
        id,
        title,
        folder,
        bytes,
    };
    cancelled(cancel)?;
    ensure!(!result.folder.exists(), Text::FolderExists);
    fs::rename(&staging, &result.folder)?;
    // A failed optional receipt must not turn a fully saved mod into a failure.
    if let Ok(receipt) = serde_json::to_vec_pretty(&result) {
        let _ = fs::write(
            runtime().join(format!("receipt-{id}-{stamp}.json")),
            receipt,
        );
    }
    Ok(result)
}

pub fn start(id: u64, destination: PathBuf, tx: Sender<Event>, cancel: Arc<AtomicBool>) {
    thread::spawn(move || {
        let result = run_download(id, &destination, &tx, &cancel);
        let event = match result {
            Ok(done) => Event::Complete(done),
            Err(_) if cancel.load(Ordering::Relaxed) => Event::Cancelled,
            Err(err) => {
                let _ = fs::write(runtime().join("last-error.log"), format!("{err:#}"));
                Event::Failed(crate::i18n::error_key(&err))
            }
        };
        let _ = tx.send(event);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_only_workshop_ids_and_trusted_urls() {
        assert_eq!(parse_id("3001909546").unwrap(), 3001909546);
        assert_eq!(
            parse_id(
                "https://steamcommunity.com/sharedfiles/filedetails/?id=3001909546&searchtext=test"
            )
            .unwrap(),
            3001909546
        );
        for input in [
            "",
            "0",
            "12 +quit",
            "https://steamcommunity.com.evil.test/sharedfiles/filedetails/?id=123",
            "http://steamcommunity.com/sharedfiles/filedetails/?id=123",
            "https://steamcommunity.com/sharedfiles/filedetails/?id=123&id=456",
            "https://steamcommunity.com/sharedfiles/filedetails/?id=18446744073709551616",
        ] {
            assert!(parse_id(input).is_err(), "accepted: {input}");
        }
    }
}
