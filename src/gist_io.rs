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
    async fn read(&self, filename: &str) -> Option<String> {
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

    pub async fn find_gist(&self, gist_name: &str) -> Result<Gist, Error> {
        let response = self.client.get(&format!("{}/gists", &self.url))
            .header("Authorization", format!("token {}", &self.git_token))
            .header("user-agent", "reqwest/0.11.1").send();
        let response_contents = match response.await {
            Ok(res) => {
                match res.json::<Vec<Gist>>().await {
                    Ok(str) => str,
                    Err(e) => {
                        println!("{}", e.to_string());
                        return Err(Error);
                    }
                }
            }
            Err(e) => return Err(Error),
        };

        for gist in response_contents {
            if gist.files.contains_key(gist_name) {
                return Ok(gist);
            }
        }
        return Err(Error)
    }

    pub async fn write_gist(&self, contents: &str, file_name: &str, gist: &Gist) -> Result<(), Error> {
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

        match response.await {
            Ok(_) => Ok(()),
            // TODO figure out how to make an error
            Err(_) => Err(Error),
        }
    }

    pub async fn read_gist(&self, file_name: &str, gist: &Gist) -> Result<String, Error> {
        match gist.read(file_name).await {
            Some(str) => Ok(str),
            None => Err(Error),
        }
    }

    pub async fn create_gist(&self, contents: &str, file_name: &str) -> Result<(), Error> {
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

        match response.await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error),
        }
    }

    fn make_gist_name(name: &str) -> String {
        format!("file_sync_{}", name)
    }

    pub async fn check_if_exists(&self, gist_name: &str) -> bool {
        match self.find_gist(gist_name).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
