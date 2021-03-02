use crate::file_io::read_file;
use std::env;

mod file_io;
mod gist_io;

fn main() {
    let gistIo = gist_io::GistIo::new(&env::var("GIT_TOKEN").unwrap());

    println!("{:?}", gistIo);
    println!("{}", gistIo.check_if_exists("file_sync_test"));
}

