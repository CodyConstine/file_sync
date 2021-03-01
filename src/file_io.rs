use std::fs;

pub fn read_file(file_name: String) -> String {
    let contents = fs::read_to_string(file_name)
        .expect("There was an error reading the file");
    contents
}

pub fn write_file(file_name: &String, body: &String) {
    fs::write(file_name, body)
        .expect("There was an error writing the file");
}