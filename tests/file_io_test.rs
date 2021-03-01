#[cfg(test)]
#[path = "../src/file_io.rs"]
mod file_io;

use std::fs;

#[test]
fn read_file() {
    let results = file_io::read_file(String::from("tests/resources/test_file"));
    let expected = String::from("this is a test file\nsecond line of test");
    assert_eq!(results, expected);
}

#[test]
fn write_file() {
    let test_file = String::from("/tmp/test_file");

    let body = String::from("this is a written test file\nsecond line of test");

    file_io::write_file(&test_file, &body);

    let results = fs::read_to_string(test_file).expect("No file");

    assert_eq!(body, results);
}