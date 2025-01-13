use anyhow::Result;
use std::borrow::Cow;
use std::path::{Path as StdPath, PathBuf};

#[derive(Debug)]
pub struct Path {
    pub(super) path: PathBuf,
}

impl Path {
    pub fn new(path: impl AsRef<StdPath>) -> Self {
        Path {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn canonicalize(&self) -> Result<Self, std::io::Error> {
        std::fs::canonicalize(&self.path).map(Self::new)
    }

    pub fn join(&self, path: impl AsRef<StdPath>) -> Path {
        Path {
            path: self.path.join(path),
        }
    }

    pub fn parent(&self) -> Option<Self> {
        self.path.parent().map(|p| Path::new(p))
    }

    pub fn to_str(&self) -> Option<&str> {
        self.path.to_str()
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
    }
}
