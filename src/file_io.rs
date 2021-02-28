use std::fs;

pub fn read_file(file_name: String) -> String {
    let contents = fs::read_to_string(file_name)
        .expect("There was an error reading file");
    contents
}