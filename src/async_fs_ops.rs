use anyhow::{bail, Result};
use std::fs::Permissions;
use tokio::fs;

use super::core::Path;

#[async_trait::async_trait]
pub trait AsyncFsOps {
    async fn chmod(&self, mode: u32) -> Result<()>;
    async fn exists(&self) -> Result<bool>;
    async fn mkdir(&self) -> Result<()>;
    async fn mkdirp(&self) -> Result<()>;
    async fn mkdirs(&self) -> Result<()>;
    async fn set_permissions(&self, permissions: Permissions) -> Result<()>;
}

#[async_trait::async_trait]
impl AsyncFsOps for Path {
    async fn chmod(&self, mode: u32) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            return Ok(fs::set_permissions(self, Permissions::from_mode(mode)).await?);
        }

        #[cfg(not(unix))]
        return bail!("chmod is not supported on this platform");
    }

    async fn exists(&self) -> Result<bool> {
        return Ok(fs::try_exists(self).await?);
    }

    async fn mkdir(&self) -> Result<()> {
        return Ok(fs::create_dir(self).await?);
    }

    async fn mkdirp(&self) -> Result<()> {
        return Ok(fs::create_dir_all(self).await?);
    }

    async fn mkdirs(&self) -> Result<()> {
        return self.mkdirp().await;
    }

    async fn set_permissions(&self, permissions: Permissions) -> Result<()> {
        return Ok(fs::set_permissions(self, permissions).await?);
    }
}
