use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec_pretty};
use std::fs::{Metadata, Permissions};
use tokio::fs::{self, OpenOptions, ReadDir};
use tokio::task::spawn_blocking;

use super::core::Path;

#[async_trait::async_trait]
pub trait AsyncFsOps {
    #[cfg(unix)]
    async fn chmod(&self, mode: u32) -> Result<()>;
    #[cfg(unix)]
    async fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> Result<()>;
    async fn create_dir(&self) -> Result<()>;
    async fn create_dir_all(&self) -> Result<()>;
    async fn empty_dir(&self) -> Result<()>;
    async fn exists(&self) -> Result<bool>;
    async fn get_file_size(&self) -> Result<u64>;
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
    async fn metadata(&self) -> Result<Metadata>;
    async fn read(&self) -> Result<Vec<u8>>;
    async fn read_dir(&self) -> Result<ReadDir>;
    async fn read_json<T: for<'de> Deserialize<'de>>(&self) -> Result<T>;
    async fn read_to_string(&self) -> Result<String>;
    async fn remove_dir(&self) -> Result<()>;
    async fn remove_dir_all(&self) -> Result<()>;
    async fn set_permissions(&self, permissions: Permissions) -> Result<()>;
    async fn truncate(&self, len: Option<u64>) -> Result<()>;
    async fn write(&self, contents: impl AsRef<[u8]> + Send) -> Result<()>;
    async fn write_json(&self, data: impl Serialize + Send) -> Result<()>;
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
        return Ok(spawn_blocking(move || std::os::unix::fs::chown(path, uid, gid)).await??);
    }

    async fn create_dir(&self) -> Result<()> {
        return Ok(fs::create_dir(self).await?);
    }

    async fn create_dir_all(&self) -> Result<()> {
        return Ok(fs::create_dir_all(self).await?);
    }

    async fn empty_dir(&self) -> Result<()> {
        if !self.exists().await? {
            return self.create_dir_all().await;
        }

        let mut entries = fs::read_dir(self).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(entry_path).await?;
            } else {
                fs::remove_file(entry_path).await?;
            }
        }

        return Ok(());
    }

    async fn exists(&self) -> Result<bool> {
        return Ok(fs::try_exists(self).await?);
    }

    async fn get_file_size(&self) -> Result<u64> {
        return Ok(self.metadata().await?.len());
    }

    #[cfg(unix)]
    async fn is_block_device(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata().await?.file_type().is_block_device());
    }

    #[cfg(unix)]
    async fn is_char_device(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata().await?.file_type().is_char_device());
    }

    async fn is_dir(&self) -> Result<bool> {
        return Ok(self.metadata().await?.is_dir());
    }

    #[cfg(unix)]
    async fn is_fifo(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata().await?.file_type().is_fifo());
    }

    async fn is_file(&self) -> Result<bool> {
        return Ok(self.metadata().await?.is_file());
    }

    #[cfg(unix)]
    async fn is_socket(&self) -> Result<bool> {
        use std::os::unix::fs::FileTypeExt;
        return Ok(self.metadata().await?.file_type().is_socket());
    }

    async fn is_symlink(&self) -> Result<bool> {
        return Ok(fs::symlink_metadata(self).await?.file_type().is_symlink());
    }

    async fn metadata(&self) -> Result<Metadata> {
        return Ok(fs::metadata(self).await?);
    }

    async fn read(&self) -> Result<Vec<u8>> {
        return Ok(fs::read(self).await?);
    }

    async fn read_dir(&self) -> Result<ReadDir> {
        return Ok(fs::read_dir(self).await?);
    }

    async fn read_json<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        return Ok(from_slice::<T>(&self.read().await?)?);
    }

    async fn read_to_string(&self) -> Result<String> {
        return Ok(fs::read_to_string(self).await?);
    }

    async fn remove_dir(&self) -> Result<()> {
        return Ok(fs::remove_dir(self).await?);
    }

    async fn remove_dir_all(&self) -> Result<()> {
        return Ok(fs::remove_dir_all(self).await?);
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

    async fn write(&self, contents: impl AsRef<[u8]> + Send) -> Result<()> {
        return Ok(fs::write(self, contents).await?);
    }

    async fn write_json(&self, data: impl Serialize + Send) -> Result<()> {
        return self.write(to_vec_pretty(&data)?).await;
    }
}
