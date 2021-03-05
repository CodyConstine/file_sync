use crate::file_io::read_file;
use std::env;
use tokio::runtime::Runtime;

mod file_io;
mod gist_io;
mod file_syncer;

fn main() {
    let gist_io = gist_io::GistIo::new(&env::var("GIT_TOKEN").unwrap());

    println!("{:?}", &gist_io);
    println!("{}", &gist_io.check_if_exists("file_sync_test"));

    let rt = Runtime::new().unwrap();
    let gist = match gist_io.find_gist("file_sync_test") {
        Some(gist) => gist,
        None => return,
    };
    gist_io.write_gist("Writing Test Data", "file_sync_test", &gist);
}

