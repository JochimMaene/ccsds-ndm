use crate::error::{CcsdsNdmError, Result};
use std::fs;
use std::path::Path;
use tempfile::Builder;

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
