use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use serde::{Deserialize, Serialize};

use super::jwt_decoder::{decode_jwt, decode_key};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub caid: i32,
    pub gid: i32,
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    rename = "Jwt bearer token",
    ty = "api_key",
    key_name = "Authorization",
    key_in = "header",
    checker = "validate_token"
)]
pub struct JwtGuard(pub User);

impl JwtGuard {
    pub fn get_user(self) -> User {
        self.0
    }
}

pub async fn validate_token(req: &Request, authorization: ApiKey) -> Option<User> {
    let public_key_base64 = req.data::<String>().unwrap();
    let decoded_key = decode_key(&public_key_base64)?;

    decode_jwt(&authorization.key, &decoded_key)
}
