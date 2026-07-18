use crate::error::{CcsdsNdmError, Result};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(crate) fn read_to_string(path: &Path, max_bytes: Option<usize>) -> Result<String> {
    let Some(limit) = max_bytes else {
        return Ok(fs::read_to_string(path)?);
    };

    let mut bytes = Vec::with_capacity(limit.min(64 * 1024).saturating_add(1));
    fs::File::open(path)?
        .take(limit.saturating_add(1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(CcsdsNdmError::ResourceLimitExceeded {
            resource: "input_document",
            limit,
            actual: bytes.len(),
        });
    }
    String::from_utf8(bytes)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error).into())
}

pub(crate) fn atomic_write(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let filename = path.file_name().ok_or_else(|| {
        CcsdsNdmError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "destination has no file name",
        ))
    })?;

    let mut temporary = None;
    let mut file = None;
    for _ in 0..16 {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut name = filename.to_os_string();
        name.push(format!(".{}.{}.tmp", std::process::id(), sequence));
        let candidate: PathBuf = parent.join(name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(opened) => {
                temporary = Some(candidate);
                file = Some(opened);
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }

    let temporary = temporary.ok_or_else(|| {
        CcsdsNdmError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "could not reserve an atomic output file",
        ))
    })?;
    let result = (|| {
        let mut file = file.expect("temporary path and file are created together");
        file.write_all(contents)?;
        file.sync_all()?;
        if let Ok(metadata) = path.metadata() {
            fs::set_permissions(&temporary, metadata.permissions())?;
        }
        fs::rename(&temporary, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}
