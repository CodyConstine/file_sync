use crate::file_io::read_file;

mod file_io;
mod gist_io;

fn main() {
    read_file(String::from("dummy"));
    println!("Hello, world!");
}

