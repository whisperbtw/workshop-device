use crate::i18n::Text;
use crate::model::Event;
use anyhow::{Result, ensure};
use std::{
    fs,
    io::{Read, Write},
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
};

struct Manifest {
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
    total: u64,
}

fn scan(root: &Path, relative: &Path, manifest: &mut Manifest, cancel: &AtomicBool) -> Result<()> {
    ensure!(!cancel.load(Ordering::Relaxed), Text::CancelledError);
    let path = root.join(relative);
    let metadata = fs::symlink_metadata(&path)?;
    ensure!(
        metadata.file_attributes() & 0x400 == 0,
        Text::UnsupportedLinks
    );
    if metadata.is_dir() {
        manifest.directories.push(relative.to_path_buf());
        for entry in fs::read_dir(path)? {
            scan(root, &relative.join(entry?.file_name()), manifest, cancel)?;
        }
    } else if metadata.is_file() {
        manifest.total += metadata.len();
        manifest.files.push(relative.to_path_buf());
    }
    Ok(())
}

/// The destination is a new private staging directory, never an existing mod.
pub fn copy(from: &Path, to: &Path, tx: &Sender<Event>, cancel: &AtomicBool) -> Result<u64> {
    let mut manifest = Manifest {
        directories: Vec::new(),
        files: Vec::new(),
        total: 0,
    };
    scan(from, Path::new(""), &mut manifest, cancel)?;
    fs::create_dir(to)?;
    for relative in manifest
        .directories
        .iter()
        .filter(|path| !path.as_os_str().is_empty())
    {
        fs::create_dir_all(to.join(relative))?;
    }
    let mut done = 0;
    let mut reported = std::time::Instant::now();
    let mut buffer = [0u8; 256 * 1024];
    for relative in manifest.files {
        let mut input = fs::File::open(from.join(&relative))?;
        let mut output = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(to.join(relative))?;
        loop {
            ensure!(!cancel.load(Ordering::Relaxed), Text::CancelledError);
            let count = input.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            output.write_all(&buffer[..count])?;
            done += count as u64;
            if done == manifest.total || reported.elapsed() >= std::time::Duration::from_millis(100)
            {
                let _ = tx.send(Event::Progress {
                    done,
                    total: manifest.total,
                });
                reported = std::time::Instant::now();
            }
        }
    }
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copy_reports_actual_bytes_and_does_not_overwrite() {
        let root = std::env::temp_dir().join(format!(
            "pz-copy-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        let source = root.join("source");
        fs::create_dir(&source).unwrap();
        fs::write(source.join("mod.txt"), b"abc").unwrap();
        let target = root.join("target");
        let (tx, rx) = std::sync::mpsc::channel();
        let cancel = AtomicBool::new(false);
        assert_eq!(copy(&source, &target, &tx, &cancel).unwrap(), 3);
        assert!(
            rx.try_iter()
                .any(|event| matches!(event, Event::Progress { done: 3, total: 3 }))
        );
        assert!(copy(&source, &target, &tx, &cancel).is_err());
        assert_eq!(fs::read(target.join("mod.txt")).unwrap(), b"abc");
        fs::remove_dir_all(root).unwrap();
    }
}
