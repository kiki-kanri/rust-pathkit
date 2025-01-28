use pathkit::Path;

#[test]
fn div_str() {
    let path = Path::new("/test/path");
    assert_eq!(&path / "subpath", Path::new("/test/path/subpath"));
    assert_eq!(path / "subpath", Path::new("/test/path/subpath"));
}

#[test]
fn div_path() {
    let path = Path::new("/test/path");
    let subpath = Path::new("subpath");
    let result_path = Path::new("/test/path/subpath");
    assert_eq!(path.clone() / subpath.clone(), result_path);
    assert_eq!(path.clone() / &subpath.clone(), result_path);
    assert_eq!(&path.clone() / subpath.clone(), result_path);
    assert_eq!(&path / &subpath, result_path);
}
