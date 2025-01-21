use anyhow::Result;
use std::fs::Permissions;
use tokio::fs::{self, OpenOptions};

use super::core::Path;

#[async_trait::async_trait]
pub trait AsyncFsOps {
    #[cfg(unix)]
    async fn chmod(&self, mode: u32) -> Result<()>;
    #[cfg(unix)]
    async fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> Result<()>;
    async fn exists(&self) -> Result<bool>;
    #[cfg(unix)]
    async fn is_block_device(&self) -> Result<bool>;
    #[cfg(unix)]
    async fn is_char_device(&self) -> Result<bool>;
    async fn is_dir(&self) -> Result<bool>;
    #[cfg(unix)]
    async fn is_fifo(&self) -> Result<bool>;
    async fn is_file(&self) -> Result<bool>;
    #[cfg(unix)]
    async fn is_socket(&self) -> Result<bool>;
    async fn is_symlink(&self) -> Result<bool>;
    async fn mkdir(&self) -> Result<()>;
    async fn mkdirp(&self) -> Result<()>;
    async fn rmdir(&self) -> Result<()>;
    async fn set_permissions(&self, permissions: Permissions) -> Result<()>;
    async fn truncate(&self, len: Option<u64>) -> Result<()>;
}

#[async_trait::async_trait]
impl AsyncFsOps for Path {
    #[cfg(unix)]
    async fn chmod(&self, mode: u32) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;
        return Ok(fs::set_permissions(self, Permissions::from_mode(mode)).await?);
    }

    #[cfg(unix)]
    async fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> Result<()> {
        let path = self.path.clone();
        return Ok(
            tokio::task::spawn_blocking(move || std::os::unix::fs::chown(path, uid, gid)).await??,
        );
    }

    async fn exists(&self) -> Result<bool> {
        return Ok(fs::try_exists(self).await?);
    }

    #[cfg(unix)]
    async fn is_block_device(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self).await?.file_type().is_block_device());
    }

    #[cfg(unix)]
    async fn is_char_device(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self).await?.file_type().is_char_device());
    }

    async fn is_dir(&self) -> Result<bool> {
        return Ok(fs::metadata(self).await?.is_dir());
    }

    #[cfg(unix)]
    async fn is_fifo(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self).await?.file_type().is_fifo());
    }

    async fn is_file(&self) -> Result<bool> {
        return Ok(fs::metadata(self).await?.is_file());
    }

    #[cfg(unix)]
    async fn is_socket(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(fs::metadata(self).await?.file_type().is_socket());
    }

    async fn is_symlink(&self) -> Result<bool> {
        return Ok(fs::symlink_metadata(self).await?.file_type().is_symlink());
    }

    async fn mkdir(&self) -> Result<()> {
        return Ok(fs::create_dir(self).await?);
    }

    async fn mkdirp(&self) -> Result<()> {
        return Ok(fs::create_dir_all(self).await?);
    }

    async fn rmdir(&self) -> Result<()> {
        return Ok(fs::remove_dir(self).await?);
    }

    async fn set_permissions(&self, permissions: Permissions) -> Result<()> {
        return Ok(fs::set_permissions(self, permissions).await?);
    }

    async fn truncate(&self, len: Option<u64>) -> Result<()> {
        return Ok(OpenOptions::new()
            .write(true)
            .open(self)
            .await?
            .set_len(len.unwrap_or(0))
            .await?);
    }
}
