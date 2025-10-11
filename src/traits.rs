use std::{
    fmt::{
        Display,
        Formatter,
    },
    path::Path as StdPath,
};

use super::core::Path;

impl AsRef<StdPath> for Path {
    fn as_ref(&self) -> &StdPath {
        &self.path
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Path {
            path: self.path.clone(),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl Eq for Path {}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
