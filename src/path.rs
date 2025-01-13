use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::path::{Path as StdPath, PathBuf};

#[derive(Debug)]
pub struct Path {
    path: PathBuf,
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

impl AsRef<StdPath> for Path {
    fn as_ref(&self) -> &StdPath {
        return &self.path;
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        return Path {
            path: self.path.clone(),
        };
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.path.to_string_lossy());
    }
}

impl Eq for Path {}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        return self.path == other.path;
    }
}
