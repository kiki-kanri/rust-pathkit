use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    path::Path as StdPath,
};

use super::core::Path;

impl AsRef<StdPath> for Path {
    fn as_ref(&self) -> &StdPath {
        &self.path
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string_lossy())
    }
}
