use crate::gist_io;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct File {
    name:String,
    location: String,
    gist_name: String,
}


impl Config {
    pub fn from_json(json_location: &str) -> Option<Config> {
        let contents = fs::read_to_string(json_location)
            .expect("Something went wrong reading the file");

        match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                println!("{}", e.to_string());
                None
            },
        }
    }

    pub fn add_file(self, name: &str, location: &str, gist_name: &str) -> Config {
        let new_file = vec![File {
            name: String::from(name),
            location: String::from(location),
            gist_name: String::from(gist_name),
        }];

        Config {
            files: [self.files, new_file].concat(),
            ..self
        }
    }
}