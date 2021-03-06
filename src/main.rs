use crate::file_io::read_file;
use std::env;
use futures::executor::block_on;
mod file_io;
mod gist_io;
mod file_syncer;

#[tokio::main]
pub async fn main() {
    let gist_io = gist_io::GistIo::new(&env::var("GIT_TOKEN").unwrap());

    let gist = match gist_io.find_gist("file_sync_test").await {
        Ok(gist) => gist,
        Err(_) => return,
    };
    gist_io.write_gist("Writing Test Data\nCody Is great", "file_sync_test", &gist).await;
}

