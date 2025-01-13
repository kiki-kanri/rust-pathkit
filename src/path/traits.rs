use std::fmt::{Display, Formatter};
use std::path::Path as StdPath;

use super::core::Path;

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
