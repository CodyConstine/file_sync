#[cfg(test)]
#[path = "../src/file_io.rs"]
mod file_io;

#[test]
fn read_file() {
    let results = file_io::read_file(String::from("tests/resources/test_file"));
    let expected = String::from("this is a test file\nsecond line of test");
    assert_eq!(results, expected);
}