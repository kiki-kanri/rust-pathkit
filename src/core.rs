use std::{
    borrow::Cow,
    ffi::OsStr,
    path::{
        Path as StdPath,
        PathBuf,
    },
};

use anyhow::Result;
use path_absolutize::Absolutize;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Path(pub(crate) PathBuf);

impl Path {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self(path.into())
    }

    pub fn absolutize(&self) -> Result<Self> {
        Ok(Self::new(self.0.absolutize()?))
    }

    pub fn absolutize_from(&self, cwd: impl AsRef<StdPath>) -> Result<Self> {
        Ok(Self::new(self.0.absolutize_from(cwd)?))
    }

    pub fn absolutize_virtually(&self, virtual_root: impl AsRef<StdPath>) -> Result<Self> {
        Ok(Self::new(self.0.absolutize_virtually(virtual_root)?))
    }

    pub fn canonicalize(&self) -> Result<Self, std::io::Error> {
        std::fs::canonicalize(&self.0).map(Self::new)
    }

    pub fn extension(&self) -> Option<&OsStr> {
        self.0.extension()
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    pub fn file_stem(&self) -> Option<&OsStr> {
        self.0.file_stem()
    }

    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn join(&self, path: impl AsRef<StdPath>) -> Self {
        Self::new(self.0.join(path))
    }

    pub fn parent(&self) -> Option<Self> {
        self.0.parent().map(Self::new)
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.0.clone()
    }

    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.0.to_string_lossy()
    }
}
