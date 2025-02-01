use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub caid: i32
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
    let server_key = req.data::<String>().unwrap();
    let decoded_key = general_purpose::STANDARD.decode(server_key).ok()?;
    let decoded_key = String::from_utf8(decoded_key).ok()?;

    let bearer_split = authorization.key.split(' ').collect::<Vec<&str>>();
    if bearer_split.len() > 2 || (bearer_split[0] != "Bearer" && bearer_split[0] != "bearer") {
        return None;
    }

    let jwt_token = bearer_split.get(1)?;

    decode::<User>(
        jwt_token,
        &DecodingKey::from_rsa_pem(decoded_key.as_ref()).ok()?,
        &Validation::new(Algorithm::RS256),
    )
    .ok()
    .map(|claims| claims.claims)
}
