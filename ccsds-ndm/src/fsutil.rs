use crate::error::{CcsdsNdmError, Result};
use std::fs;
use std::io::Read;
use std::path::Path;
use tempfile::Builder;

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

pub(crate) fn atomic_write(
    path: &Path,
    write: impl FnOnce(&mut fs::File) -> Result<()>,
) -> Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    path.file_name().ok_or_else(|| {
        CcsdsNdmError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "destination has no file name",
        ))
    })?;

    let mut builder = Builder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        builder.permissions(fs::Permissions::from_mode(0o666));
    }
    let mut temporary = builder.tempfile_in(parent)?;
    write(temporary.as_file_mut())?;
    temporary.as_file().sync_all()?;
    if let Ok(metadata) = path.metadata() {
        temporary
            .as_file()
            .set_permissions(metadata.permissions())?;
    }
    temporary.persist(path).map_err(std::io::Error::from)?;
    Ok(())
}
