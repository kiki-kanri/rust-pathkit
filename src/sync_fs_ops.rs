use anyhow::Result;
use std::fs::{self, Metadata, OpenOptions, Permissions};

use super::core::Path;

pub trait SyncFsOps {
    #[cfg(unix)]
    fn chmod_sync(&self, mode: u32) -> Result<()>;
    #[cfg(unix)]
    fn chown_sync(&self, uid: Option<u32>, gid: Option<u32>) -> Result<()>;
    fn create_dir_all_sync(&self) -> Result<()>;
    fn create_dir_sync(&self) -> Result<()>;
    fn empty_dir_sync(&self) -> Result<()>;
    fn exists_sync(&self) -> Result<bool>;
    fn get_file_size_sync(&self) -> Result<u64>;
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
    fn metadata_sync(&self) -> Result<Metadata>;
    fn remove_dir_all_sync(&self) -> Result<()>;
    fn remove_dir_sync(&self) -> Result<()>;
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

    fn create_dir_all_sync(&self) -> Result<()> {
        return Ok(fs::create_dir_all(self)?);
    }

    fn create_dir_sync(&self) -> Result<()> {
        return Ok(fs::create_dir(self)?);
    }

    fn empty_dir_sync(&self) -> Result<()> {
        if !self.exists_sync()? {
            return self.create_dir_all_sync();
        }

        for entry in fs::read_dir(self)? {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path)?;
            } else {
                fs::remove_file(entry_path)?;
            }
        }

        return Ok(());
    }

    fn exists_sync(&self) -> Result<bool> {
        return Ok(self.path.try_exists()?);
    }

    fn get_file_size_sync(&self) -> Result<u64> {
        return Ok(self.metadata_sync()?.len());
    }

    #[cfg(unix)]
    fn is_block_device_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata_sync()?.file_type().is_block_device());
    }

    #[cfg(unix)]
    fn is_char_device_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata_sync()?.file_type().is_char_device());
    }

    fn is_dir_sync(&self) -> Result<bool> {
        return Ok(self.metadata_sync()?.is_dir());
    }

    #[cfg(unix)]
    fn is_fifo_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata_sync()?.file_type().is_fifo());
    }

    fn is_file_sync(&self) -> Result<bool> {
        return Ok(self.metadata_sync()?.is_file());
    }

    #[cfg(unix)]
    fn is_socket_sync(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata_sync()?.file_type().is_socket());
    }

    fn is_symlink_sync(&self) -> Result<bool> {
        return Ok(fs::symlink_metadata(self)?.file_type().is_symlink());
    }

    fn metadata_sync(&self) -> Result<Metadata> {
        return Ok(fs::metadata(self)?);
    }

    fn remove_dir_all_sync(&self) -> Result<()> {
        return Ok(fs::remove_dir_all(self)?);
    }

    fn remove_dir_sync(&self) -> Result<()> {
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
