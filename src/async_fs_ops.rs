use anyhow::Result;
use tokio::fs;

use super::core::Path;

#[async_trait::async_trait]
pub trait AsyncFsOps {
    async fn exists(&self) -> Result<bool>;
    async fn mkdir(&self) -> Result<()>;
    async fn mkdirp(&self) -> Result<()>;
    async fn mkdirs(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl AsyncFsOps for Path {
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
}
