use reqwest;
use serde_json;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct HTTPError {
    pub status: u16,
    pub body: String,
}

impl fmt::Display for HTTPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status {} : {}", self.status, self.body)
    }
}

impl error::Error for HTTPError {}

#[derive(Debug)]
pub enum Error {
    HTTP(HTTPError),
    Reqwest(reqwest::Error),
    Serde(serde_json::error::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HTTP(e) => write!(f, "{}", e),
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Serde(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {}

impl From<HTTPError> for Error {
    fn from(err: HTTPError) -> Error {
        Error::HTTP(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::Serde(err)
    }
}
