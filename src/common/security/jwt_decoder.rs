use super::jwt_guard::User;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub fn decode_jwt(authorization_header: &str, jwt_public_key: &str) -> Option<User> {
    let bearer_split = authorization_header.split(' ').collect::<Vec<&str>>();
    if bearer_split.len() > 2 || (bearer_split[0] != "Bearer" && bearer_split[0] != "bearer") {
        return None;
    }

    let jwt_token = bearer_split.get(1)?;

    decode::<User>(
        &jwt_token,
        &DecodingKey::from_rsa_pem(jwt_public_key.as_ref()).ok()?,
        &Validation::new(Algorithm::RS256),
    )
    .ok()
    .map(|claims| claims.claims)
}
