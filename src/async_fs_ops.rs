use anyhow::{anyhow, Result};
use tokio::fs;

use super::core::Path;

pub trait AsyncFsOps {
    async fn exists(&self) -> Result<bool>;
    async fn mkdir(&self) -> Result<()>;
    async fn mkdirp(&self) -> Result<()>;
    async fn mkdirs(&self) -> Result<()>;
}

impl AsyncFsOps for Path {
    async fn exists(&self) -> Result<bool> {
        return fs::try_exists(self).await;
    }

    async fn mkdir(&self) -> Result<()> {
        fs::create_dir(self).await?;
        return Ok(());
    }

    async fn mkdirp(&self) -> Result<()> {
        fs::create_dir_all(self).await?;
        return Ok(());
    }

    async fn mkdirs(&self) -> Result<()> {
        return self.mkdirp().await;
    }
}
