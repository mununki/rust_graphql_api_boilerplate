extern crate jsonwebtoken as jwt;
extern crate serde_derive;

use jwt::{decode, encode, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: i32,
}

struct SecretKey {
    key: &'static str,
}

impl SecretKey {
    fn get_secret_key() -> SecretKey {
        SecretKey { key: "secret" }
    }
}

pub fn encode_jwt(user_id: i32, exp_day: i32) -> Result<String, jwt::errors::Error> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i32
        + exp_day * 24 * 60 * 60;
    let jwt_secret_key = SecretKey::get_secret_key();

    let my_claims = Claims { user_id, exp };

    let token = encode(&Header::default(), &my_claims, jwt_secret_key.key.as_ref())?;

    Ok(token)
}

pub fn verify_jwt(token: String) -> Result<jwt::TokenData<Claims>, jwt::errors::Error> {
    let jwt_secret_key = SecretKey::get_secret_key();

    let validation = Validation::default();

    match decode::<Claims>(&token, jwt_secret_key.key.as_ref(), &validation) {
        Ok(c) => Ok(c),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_token() {
        let token = match encode_jwt(1, 30) {
            Ok(t) => t,
            _ => return println!("Test failed!"),
        };
        println!("token: {:?}", &token);
        println!("output: {:?}", verify_jwt(token))
    }
}
