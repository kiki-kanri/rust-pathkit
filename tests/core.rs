use anyhow::Result;
use pathkit::Path;
use std::ffi::OsStr;

#[test]
fn new_creates_path_instance() {
    let path = Path::new("/test/path");
    assert_eq!(path.to_str(), Some("/test/path"));
}

#[test]
fn absolutize_resolves_relative_path() -> Result<()> {
    let path = Path::new(".");
    assert!(path.absolutize()?.is_absolute());
    return Ok(());
}

#[test]
fn absolutize_from_uses_specified_cwd() -> Result<()> {
    let path = Path::new("subdir");
    assert_eq!(
        path.absolutize_from("/base")?.to_str(),
        Some("/base/subdir")
    );

    return Ok(());
}

#[test]
fn absolutize_virtually_uses_virtual_root() -> Result<()> {
    let path = Path::new("subdir/file.txt");
    assert_eq!(
        path.absolutize_virtually("/virtual")?.to_str(),
        Some("/virtual/subdir/file.txt")
    );

    return Ok(());
}

#[test]
fn canonicalize_returns_absolute_path() -> Result<()> {
    let path = Path::new(".");
    assert!(path.canonicalize()?.is_absolute());
    return Ok(());
}

#[test]
fn extension_returns_file_extension() {
    let path = Path::new("file.txt");
    assert_eq!(path.extension(), Some(OsStr::new("txt")));
}

#[test]
fn file_name_returns_last_segment() {
    let path = Path::new("/path/to/file.txt");
    assert_eq!(path.file_name(), Some(OsStr::new("file.txt")));
}

#[test]
fn file_stem_returns_file_name_without_extension() {
    let path = Path::new("file.txt");
    assert_eq!(path.file_stem(), Some(OsStr::new("file")));
}

#[test]
fn is_absolute_checks_if_path_is_absolute() {
    let path = Path::new("/absolute/path");
    assert!(path.is_absolute());
    let relative_path = Path::new("relative/path");
    assert!(!relative_path.is_absolute());
}

#[test]
fn join_combines_paths() {
    let path = Path::new("/base");
    assert_eq!(path.join("subdir").to_str(), Some("/base/subdir"));
}

#[test]
fn parent_returns_parent_directory() {
    let path = Path::new("/base/subdir/file.txt");
    assert_eq!(path.parent().unwrap().to_str(), Some("/base/subdir"));
}

#[test]
fn to_str_converts_path_to_str() {
    let path = Path::new("/test/path");
    assert_eq!(path.to_str(), Some("/test/path"));
}

#[test]
fn to_string_lossy_returns_lossy_string() {
    let path = Path::new("/test/path");
    assert_eq!(path.to_string_lossy(), "/test/path");
}