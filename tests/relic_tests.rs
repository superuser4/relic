use tempfile::{tempdir, NamedTempFile};
use std::io::Write;
use relic::count_dir;

#[test]
fn test_single_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "line1\nline2\nline3").unwrap();

    let lines = count_dir(String::from(file.path().to_str().unwrap()), &None).unwrap();
    assert_eq!(lines, 3);
}

#[test]
fn test_empty_file() {
    let file = NamedTempFile::new().unwrap();

    let lines = count_dir( String::from(file.path().to_str().unwrap()), &None).unwrap();
    assert_eq!(lines, 0);
}

#[test]
fn test_directory_multiple_files() {
    let dir = tempdir().unwrap();

    // file1
    let mut f1 = std::fs::File::create(dir.path().join("a.c")).unwrap();
    writeln!(f1, "a\nb\nc").unwrap();

    // file2
    let mut f2 = std::fs::File::create(dir.path().join("b.c")).unwrap();
    writeln!(f2, "1\n2").unwrap();

    let total = count_dir(String::from(dir.path().to_str().unwrap()), &Some(String::from("c"))).unwrap();
    assert_eq!(total, 5);
}

#[test]
fn test_extension_filter_ignores_other_files() {
    let dir = tempdir().unwrap();

    // matches
    let mut f1 = std::fs::File::create(dir.path().join("a.c")).unwrap();
    writeln!(f1, "x").unwrap();

    // does NOT match
    let mut f2 = std::fs::File::create(dir.path().join("b.txt")).unwrap();
    writeln!(f2, "ignored").unwrap();

    let total = count_dir(String::from(dir.path().to_str().unwrap()), &Some(String::from("c"))).unwrap();
    assert_eq!(total, 1);
}

#[test]
fn test_no_matching_files() {
    let dir = tempdir().unwrap();
    let total = count_dir(String::from(dir.path().to_str().unwrap()), &Some(String::from("rs"))).unwrap();
    assert_eq!(total, 0);
}

