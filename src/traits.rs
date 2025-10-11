use std::{
    borrow::Borrow,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    ops::Deref,
    path::{
        Path as StdPath,
        PathBuf,
    },
};

use super::core::Path;

impl AsRef<StdPath> for Path {
    fn as_ref(&self) -> &StdPath {
        &self.0
    }
}

impl Borrow<StdPath> for Path {
    fn borrow(&self) -> &StdPath {
        &self.0
    }
}

impl Deref for Path {
    type Target = StdPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl From<&StdPath> for Path {
    fn from(path: &StdPath) -> Self {
        Self(path.to_path_buf())
    }
}

impl From<Path> for PathBuf {
    fn from(path: Path) -> Self {
        path.0
    }
}
