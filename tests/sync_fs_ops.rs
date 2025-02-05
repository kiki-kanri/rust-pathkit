use anyhow::Result;
use std::fs;
use tempfile::NamedTempFile;

use pathkit::{Path, SyncFsOps};

#[cfg(unix)]
#[test]
fn chmod_sync() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let temp_file = NamedTempFile::new()?;
    let file_path = Path::new(temp_file.path());
    file_path.chmod_sync(0o744)?;
    assert_eq!(
        fs::metadata(file_path.clone())?.permissions().mode() & 0o777,
        0o744
    );

    file_path.chmod_sync(0o700)?;
    assert_eq!(
        fs::metadata(file_path.clone())?.permissions().mode() & 0o777,
        0o700
    );

    return Ok(());
}
