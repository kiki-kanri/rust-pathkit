use anyhow::Result;
use std::fs::{self, OpenOptions, Permissions};

use super::core::Path;

pub trait SyncFsOps {
    #[cfg(unix)]
    fn chmod_sync(&self, mode: u32) -> Result<()>;
    #[cfg(unix)]
    fn chown_sync(&self, uid: Option<u32>, gid: Option<u32>) -> Result<()>;
    fn exists_sync(&self) -> Result<bool>;
    #[cfg(unix)]
    fn is_block_device_sync(&self) -> Result<bool>;
    #[cfg(unix)]
    fn is_char_device_sync(&self) -> Result<bool>;
    fn is_dir_sync(&self) -> Result<bool>;
    #[cfg(unix)]
    fn is_fifo_sync(&self) -> Result<bool>;
    fn is_file_sync(&self) -> Result<bool>;
    #[cfg(unix)]
    fn is_socket_sync(&self) -> Result<bool>;
    fn is_symlink_sync(&self) -> Result<bool>;
    fn mkdir_sync(&self) -> Result<()>;
    fn mkdirp_sync(&self) -> Result<()>;
    fn rmdir_sync(&self) -> Result<()>;
    fn set_permissions_sync(&self, permissions: Permissions) -> Result<()>;
    fn truncate_sync(&self, len: Option<u64>) -> Result<()>;
}

impl SyncFsOps for Path {
    #[cfg(unix)]
    fn chmod_sync(&self, mode: u32) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;
        return Ok(fs::set_permissions(self, Permissions::from_mode(mode))?);
    }

    #[cfg(unix)]
    fn chown_sync(&self, uid: Option<u32>, gid: Option<u32>) -> Result<()> {
        return Ok(std::os::unix::fs::chown(self, uid, gid)?);
    }

    fn exists_sync(&self) -> Result<bool> {
        return Ok(self.path.try_exists()?);
    }

    #[cfg(unix)]
    fn is_block_device_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self)?.file_type().is_block_device());
    }

    #[cfg(unix)]
    fn is_char_device_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self)?.file_type().is_char_device());
    }

    fn is_dir_sync(&self) -> Result<bool> {
        return Ok(fs::metadata(self)?.is_dir());
    }

    #[cfg(unix)]
    fn is_fifo_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self)?.file_type().is_fifo());
    }

    fn is_file_sync(&self) -> Result<bool> {
        return Ok(fs::metadata(self)?.is_file());
    }

    #[cfg(unix)]
    fn is_socket_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self)?.file_type().is_socket());
    }

    fn is_symlink_sync(&self) -> Result<bool> {
        return Ok(fs::symlink_metadata(self)?.file_type().is_symlink());
    }

    fn mkdir_sync(&self) -> Result<()> {
        return Ok(fs::create_dir(self)?);
    }

    fn mkdirp_sync(&self) -> Result<()> {
        return Ok(fs::create_dir_all(self)?);
    }

    fn rmdir_sync(&self) -> Result<()> {
        return Ok(fs::remove_dir(self)?);
    }

    fn set_permissions_sync(&self, permissions: Permissions) -> Result<()> {
        return Ok(fs::set_permissions(self, permissions)?);
    }

    fn truncate_sync(&self, len: Option<u64>) -> Result<()> {
        return Ok(OpenOptions::new()
            .write(true)
            .open(self)?
            .set_len(len.unwrap_or(0))?);
    }
}
