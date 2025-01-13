use std::borrow::Cow;
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

    pub fn to_str(&self) -> Option<&str> {
        return self.path.to_str();
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        return self.path.to_string_lossy();
    }
}
