use pathkit::Path;

#[test]
fn as_ref() {
    let path = Path::new("/test/path");
    assert_eq!(path.as_ref().to_str(), Some("/test/path"));
}

#[test]
fn clone() {
    let path = Path::new("/test/path");
    assert_eq!(path.clone().to_str(), Some("/test/path"));
}

#[test]
fn fmt() {
    let path = Path::new("/test/path");
    assert_eq!(format!("{}", path), "/test/path");
}

#[test]
fn eq() {
    let path = Path::new("/test/path");
    assert_eq!(path, Path::new("/test/path"));
}
