use crate::user;
use crate::client;
use crate::client::HTTPClient;
use crate::error;
use serde::{Serialize, Deserialize};
use serde_json;

type Name = String;
type Validity = i32;

#[derive(Debug, Serialize)]
pub struct CreateToken {
    pub name: Name,
    pub password: user::Password,
    pub user: user::Name,
    pub validity: Validity,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenResponse {
    pub token: String
}

impl CreateToken {
    pub fn new(name: Name, password: user::Password, user: user::Name, validity: Validity) -> CreateToken {

        CreateToken {
            name: name,
            password: password,
            user: user,
            validity: validity
        }
    }
}

pub fn create_token(client: client::Client, req: &CreateToken) -> Result<String, error::Error> {
    let json_body = serde_json::to_string(req)?;
    let response = client.post("/api/v1/meuse/token", json_body)?;
    let body = response.text()?;
    Ok(body)
}
