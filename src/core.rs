use anyhow::Result;
use path_absolutize::Absolutize;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path as StdPath, PathBuf};

#[derive(Debug)]
pub struct Path {
    pub(super) path: PathBuf,
}

impl Path {
    pub fn new(path: impl AsRef<StdPath>) -> Self {
        return Path {
            path: path.as_ref().to_path_buf(),
        };
    }

    pub fn absolutize(&self) -> Result<Self> {
        return Ok(Self::new(self.path.absolutize()?));
    }

    pub fn absolutize_from(&self, cwd: impl AsRef<StdPath>) -> Result<Self> {
        return Ok(Self::new(self.path.absolutize_from(cwd)?));
    }

    pub fn absolutize_virtually(&self, virtual_root: impl AsRef<StdPath>) -> Result<Self> {
        return Ok(Self::new(self.path.absolutize_virtually(virtual_root)?));
    }

    pub fn canonicalize(&self) -> Result<Self, std::io::Error> {
        return std::fs::canonicalize(&self.path).map(Self::new);
    }

    pub fn extension(&self) -> Option<&OsStr> {
        return self.path.extension();
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        return self.path.file_name();
    }

    pub fn file_stem(&self) -> Option<&OsStr> {
        return self.path.file_stem();
    }

    pub fn is_absolute(&self) -> bool {
        return self.path.is_absolute();
    }

    pub fn join(&self, path: impl AsRef<StdPath>) -> Self {
        return Self::new(self.path.join(path));
    }

    pub fn parent(&self) -> Option<Self> {
        return self.path.parent().map(Self::new);
    }

    pub fn to_str(&self) -> Option<&str> {
        return self.path.to_str();
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        return self.path.to_string_lossy();
    }
}
