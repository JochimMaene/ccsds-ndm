use crate::error::{CcsdsNdmError, Result};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use tempfile::Builder;

pub(crate) fn atomic_write(
    path: &Path,
    write: impl FnOnce(&mut BufWriter<&mut fs::File>) -> Result<()>,
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
    {
        let mut output = BufWriter::new(temporary.as_file_mut());
        write(&mut output)?;
        output.flush()?;
    }
    temporary.as_file().sync_all()?;
    if let Ok(metadata) = path.metadata() {
        temporary
            .as_file()
            .set_permissions(metadata.permissions())?;
    }
    temporary.persist(path).map_err(std::io::Error::from)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{ErrorKind, Write};

    #[test]
    fn successful_write_flushes_the_buffer_before_replacing_the_file() {
        let directory = tempfile::tempdir().unwrap();
        let destination = directory.path().join("message");
        fs::write(&destination, b"original").unwrap();
        let expected = "new content\n".repeat(1_000);
        atomic_write(&destination, |output| {
            for byte in expected.bytes() {
                output.write_all(&[byte])?;
            }
            Ok(())
        })
        .unwrap();
        assert_eq!(fs::read_to_string(&destination).unwrap(), expected);
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }

    #[test]
    fn failed_write_discards_partial_output_and_cleans_up() {
        for existing in [false, true] {
            let directory = tempfile::tempdir().unwrap();
            let destination = directory.path().join("message");
            if existing {
                fs::write(&destination, b"original").unwrap();
            }
            let error = atomic_write(&destination, |output| {
                output.write_all(b"partial document")?;
                Err(std::io::Error::other("deliberate failure").into())
            })
            .unwrap_err();
            assert!(matches!(error, CcsdsNdmError::Io(ref error)
                if error.kind() == ErrorKind::Other && error.to_string() == "deliberate failure"));
            if existing {
                assert_eq!(fs::read(&destination).unwrap(), b"original");
            } else {
                assert!(!destination.exists());
            }
            assert_eq!(
                fs::read_dir(directory.path()).unwrap().count(),
                usize::from(existing)
            );
        }
    }
}
