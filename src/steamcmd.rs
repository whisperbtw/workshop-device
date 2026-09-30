use crate::i18n::Text;
use anyhow::{Context, Result, bail, ensure};
use std::os::windows::io::AsRawHandle;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    os::windows::process::CommandExt,
    path::Path,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::JobObjects::*,
};

const NO_WINDOW: u32 = 0x08000000;

/// Every error path releases the child, including log or pipe failures.
struct Job(HANDLE);

impl Job {
    fn new() -> Result<Self> {
        let job = Self(unsafe { CreateJobObjectW(None, None)? });
        let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        unsafe {
            SetInformationJobObject(
                job.0,
                JobObjectExtendedLimitInformation,
                &limits as *const _ as _,
                std::mem::size_of_val(&limits) as u32,
            )?;
        }
        Ok(job)
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

struct Process {
    child: Child,
    job: Job,
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            let _ = TerminateJobObject(self.job.0, 1);
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn confirms(line: &str, id: u64) -> bool {
    let mut words = line.split_whitespace();
    words.next() == Some("Success.")
        && words.next() == Some("Downloaded")
        && words.next() == Some("item")
        && words.next().and_then(|word| word.parse::<u64>().ok()) == Some(id)
}

pub fn download(
    exe: &Path,
    cache: &Path,
    app_id: u32,
    id: u64,
    log_path: &Path,
    cancel: &AtomicBool,
) -> Result<()> {
    let mut log = File::create(log_path)?;
    let job = Job::new().context(Text::PrepareFailed)?;
    let child = Command::new(exe)
        .current_dir(cache)
        .creation_flags(NO_WINDOW)
        .args([
            "+login",
            "anonymous",
            "+workshop_download_item",
            &app_id.to_string(),
            &id.to_string(),
            "validate",
            "+quit",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
        .context(Text::StartFailed)?;
    let mut process = Process { child, job };
    unsafe { AssignProcessToJobObject(process.job.0, HANDLE(process.child.as_raw_handle())) }
        .context(Text::ControlFailed)?;
    let (tx, rx) = mpsc::sync_channel::<String>(256);
    let pipes: [Box<dyn Read + Send>; 2] = [
        Box::new(process.child.stdout.take().context(Text::OutputFailed)?),
        Box::new(process.child.stderr.take().context(Text::OutputFailed)?),
    ];
    for pipe in pipes {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut reader = BufReader::new(pipe);
            let mut bytes = Vec::new();
            loop {
                bytes.clear();
                match reader.by_ref().take(65536).read_until(b'\n', &mut bytes) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        if tx
                            .send(String::from_utf8_lossy(&bytes).trim_end().to_owned())
                            .is_err()
                        {
                            break;
                        }
                    }
                }
            }
        });
    }
    drop(tx);
    let mut confirmed = false;
    let started = Instant::now();
    loop {
        ensure!(!cancel.load(Ordering::Relaxed), Text::CancelledError);
        ensure!(started.elapsed() < Duration::from_secs(1800), Text::Timeout);
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(line) => {
                writeln!(log, "{line}")?;
                confirmed |= confirms(&line, id);
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        // SteamCMD may re-execute during preparation. Pipe EOF, not the first
        // process exit, determines when its complete output has been received.
        let _ = process.child.try_wait()?;
    }
    if !confirmed {
        bail!(Text::AnonymousDenied);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn confirmation_matches_the_exact_requested_id() {
        assert!(confirms("Success. Downloaded item 123 to some/path", 123));
        assert!(!confirms("Success. Downloaded item 1234", 123));
        assert!(!confirms("ERROR! Downloaded item 123", 123));
    }
}
