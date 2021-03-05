use reqwest::Client;
use serde_json::json;
use std::fmt::Error;
use tokio::runtime::Runtime;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GistIo {
    client: Client,
    url: String,
    git_token: String,
}

#[derive(Deserialize, Debug)]
pub struct File {
    filename: String,
    raw_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Gist {
    id: String,
    files: HashMap<String, File>,
}

impl Gist {
    async fn read(self, filename: &str) -> Option<String> {
        let url = match self.files.get(filename) {
            Some(file) => &file.raw_url,
            None => return None,
        };
        let response = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(_) => return None

        };

        match response.text().await {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }
}

impl GistIo {
    pub fn new(token: &str) -> GistIo {
        let client = Client::new();
        let url: String = String::from("https://api.github.com");
        GistIo {
            client,
            url,
            git_token: String::from(token),
        }
    }

    pub fn find_gist(&self, gist_name: &str) -> Option<Gist> {
        let rt = Runtime::new().unwrap();
        let resp_built = self.client.get(&format!("{}/gists", &self.url))
            .header("Authorization", format!("token {}", &self.git_token))
            .header("user-agent", "reqwest/0.11.1");
        let response = rt.block_on(resp_built.send());
        let response_contents = match response {
            Ok(res) => {
                match rt.block_on(res.json::<Vec<Gist>>()) {
                    Ok(str) => str,
                    Err(e) => {
                        println!("{}", e.to_string());
                        return None;
                    }
                }
            }
            Err(e) => return None,
        };

        for gist in response_contents {
            if gist.files.contains_key(gist_name) {
                return Some(gist);
            }
        }
        return None;
    }

    pub fn write_gist(&self, contents: &str, file_name: &str, gist: &Gist) -> Result<(), Error> {
        let gist_body = json!({
        "files": {
             file_name: {
             "content": contents
            }
        }});
        let response = self.client.patch(&format!("{}/gists/{}", &self.url, gist.id))
            .header("Authorization", format!("token {}", &self.git_token))
            .header("user-agent", "reqwest/0.11.1")
            .json(&gist_body).send();

        let rt = Runtime::new().unwrap();
        match rt.block_on(response) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error),
        }
    }

    pub fn create_gist(&self, contents: &str, file_name: &str) -> Result<(), Error> {
        let gist_body = json!({
        "files": {
             file_name: {
             "content": contents
            }
        }});
        let response = self.client.post(&format!("{}/gists", &self.url))
            .header("Authorization", format!("token {}", &self.git_token))
            .header("user-agent", "reqwest/0.11.1")
            .json(&gist_body).send();

        let rt = Runtime::new().unwrap();
        match rt.block_on(response) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error),
        }
    }

    fn make_gist_name(name: &str) -> String {
        format!("file_sync_{}", name)
    }

    pub fn check_if_exists(&self, gist_name: &str) -> bool {
        self.find_gist(gist_name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::gist_io::GistIo;
    use reqwest::Client;

    #[test]
    fn test_make_gist_name() {
        let results = GistIo::make_gist_name("test_file");
        let expected = String::from("file_sync_test_file");

        assert_eq!(results, expected);
    }

    #[test]
    fn test_check_if_exists() {
        let url = mockito::server_url();
        let gist_name = String::from("test_gist");
        let client = Client::new();
        let token = String::from("token");
        let test = GistIo {
            client,
            url,
            git_token: token,
        };

        let _m = mockito::mock("GET", "/gists")
            .with_status(201)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body(r#"
                [
                  {
                    "id": "aa5a315d61ae9438b18d",
                    "files": {
                      "hello_world.rb": {
                        "filename": "hello_world.rb"
                      }
                    },
                    "truncated": false
                  },
                  {
                    "id": "aa5a315d61ae9438b18d",
                    "files": {
                      "test_gist": {
                        "filename": "test_gist"
                      }
                    },
                    "truncated": false
                  }
                ]
            "#)
            .create();

        let results = test.check_if_exists("test_gist");

        assert!(results);
    }

    #[test]
    fn test_check_if_does_not_exists() {
        let url = mockito::server_url();
        let client = Client::new();
        let token = String::from("token");
        let test = GistIo {
            client,
            url,
            git_token: token,
        };

        let _m = mockito::mock("GET", "/gists")
            .with_status(201)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body(r#"
                [
                  {
                    "id": "aa5a315d61ae9438b18d",
                    "files": {
                      "hello_world.rb": {
                        "filename": "hello_world.rb"
                      }
                    },
                    "truncated": false
                  },
                  {
                    "id": "aa5a315d61ae9438b18d",
                    "files": {
                      "test_gist": {
                        "filename": "test_gist"
                      }
                    },
                    "truncated": false
                  }
                ]
            "#)
            .create();

        let results = test.check_if_exists("test_gist_not_real");

        assert!(!results);
    }

    #[test]
    fn test_find_gist() {
        let url = mockito::server_url();
        let gist_name = String::from("test_gist");
        let client = Client::new();
        let token = String::from("token");
        let test = GistIo {
            client,
            url,
            git_token: token,
        };
        let _m = mockito::mock("GET", "/gists")
            .with_status(201)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body(r#"
                [
                  {
                    "id": "aa5a315d61ae9438b18d",
                    "files": {
                      "hello_world.rb": {
                        "filename": "hello_world.rb"
                      }
                    },
                    "truncated": false
                  },
                  {
                    "id": "aa5a315d61ae9438b18d",
                    "files": {
                      "test_gist": {
                        "filename": "test_gist"
                      }
                    },
                    "truncated": false
                  }
                ]
            "#)
            .create();
        let results = match test.find_gist(&gist_name) {
            Some(str) => str.id,
            None => String::from("NONE"),
        };
        let expected = String::from("aa5a315d61ae9438b18d");
        assert_eq!(results, expected)
    }
}