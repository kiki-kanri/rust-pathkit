use std::ops::Div;

use super::core::Path;

impl Div<&str> for Path {
    type Output = Path;

    fn div(self, rhs: &str) -> Self::Output {
        return self.join(rhs);
    }
}

impl Div<&str> for &Path {
    type Output = Path;

    fn div(self, rhs: &str) -> Self::Output {
        return self.join(rhs);
    }
}

impl Div<Path> for Path {
    type Output = Path;

    fn div(self, rhs: Path) -> Self::Output {
        return self.join(rhs.as_ref());
    }
}

impl Div<&Path> for Path {
    type Output = Path;

    fn div(self, rhs: &Path) -> Self::Output {
        return self.join(rhs.as_ref());
    }
}

impl Div<Path> for &Path {
    type Output = Path;

    fn div(self, rhs: Path) -> Self::Output {
        return self.join(rhs.as_ref());
    }
}

impl Div<&Path> for &Path {
    type Output = Path;

    fn div(self, rhs: &Path) -> Self::Output {
        return self.join(rhs.as_ref());
    }
}
