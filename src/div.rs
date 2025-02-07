use std::ops::Div;

use super::core::Path;

impl Div<&Path> for &Path {
    type Output = Path;

    fn div(self, rhs: &Path) -> Self::Output {
        return self.join(rhs);
    }
}

impl Div<&Path> for Path {
    type Output = Path;

    fn div(self, rhs: &Path) -> Self::Output {
        return self.join(rhs);
    }
}

impl Div<Path> for &Path {
    type Output = Path;

    fn div(self, rhs: Path) -> Self::Output {
        return self.join(&rhs);
    }
}

impl Div<Path> for Path {
    type Output = Path;

    fn div(self, rhs: Path) -> Self::Output {
        return self.join(&rhs);
    }
}

impl<T: AsRef<str>> Div<T> for &Path {
    type Output = Path;

    fn div(self, rhs: T) -> Self::Output {
        return self.join(rhs.as_ref());
    }
}

impl<T: AsRef<str>> Div<T> for Path {
    type Output = Path;

    fn div(self, rhs: T) -> Self::Output {
        return self.join(rhs.as_ref());
    }
}
