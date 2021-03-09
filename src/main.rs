use crate::file_io::read_file;
use std::env;
use futures::executor::block_on;
use crate::file_syncer::Config;

mod file_io;
mod gist_io;
mod file_syncer;

pub fn main() {
    let config = match Config::from_json("test.json") {
        Some(config) => config,
        None => return
    };
    println!("{:?}", config);

    let new_config = config.add_file(".bashrc", "/home/cody/.bashrc", ".bashrc");
    println!("{:?}", new_config);
}


