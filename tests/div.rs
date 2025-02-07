use pathkit::Path;

#[test]
fn div_path() {
    let path = Path::new("/test/path");
    let subpath = Path::new("subpath");
    let result_path = Path::new("/test/path/subpath");
    assert_eq!(&path / &subpath, result_path);
    assert_eq!(path.clone() / &subpath, result_path);
    assert_eq!(&path / subpath.clone(), result_path);
    assert_eq!(path / subpath, result_path);
}

#[test]
fn div_as_ref_str() {
    let path = Path::new("/test/path");
    let subpath = "subpath";
    let subpath_string = "subpath".to_string();
    let result_path = Path::new("/test/path/subpath");
    assert_eq!(&path / subpath, result_path);
    assert_eq!(&path / subpath_string.clone(), result_path);
    assert_eq!(&path / &subpath_string, result_path);
    assert_eq!(path.clone() / subpath, result_path);
    assert_eq!(path.clone() / subpath_string.clone(), result_path);
    assert_eq!(path / &subpath_string, result_path);
}
