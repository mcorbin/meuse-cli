use crate::error;
use reqwest;

pub trait HTTPClient {
    fn post(&self, path: &str, body: String) -> Result<reqwest::blocking::Response, error::Error>;
    fn get(&self, path: &str) -> Result<reqwest::blocking::Response, error::Error>;
}

#[derive(Debug)]
pub struct Client {
    pub url: String,
    pub client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(proto: &str,
               url: &str,
               port: i32,
               client: reqwest::blocking::Client) -> Client {
        Client{
            url: format!("{}://{}:{}", proto, url, port),
            client: client
        }
    }
}

pub fn check_response(response: Result<reqwest::blocking::Response, reqwest::Error>)
                      -> Result<reqwest::blocking::Response, error::Error> {
    match response {
        Ok(r) => {
            if r.status().is_success() {
                Ok(r)
            }
            else {
                let status = r.status().as_u16();
                let error_body = r.text()?;
                Err(error::Error::HTTP(error::HTTPError{
                    status: status,
                    body: error_body}))
            }
        },
        Err(e) => {
            Err(error::Error::Reqwest(e))
        }
    }
}

impl HTTPClient for Client {
    fn post(&self, path: &str, body: String)
            -> Result<reqwest::blocking::Response, error::Error> {
        let req_url = format!("{}{}", self.url, path);
        let response = self.client.post(&req_url).body(body).send();
        check_response(response)
    }

    fn get(&self, path: &str)
           -> Result<reqwest::blocking::Response, error::Error> {
        let req_url = format!("{}{}", self.url, path);
        let response = self.client.get(&req_url).send();
        check_response(response)
    }

}
