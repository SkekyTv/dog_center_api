use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Result as JwtResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user id or email)
    pub exp: usize,  // expiration timestamp
}

pub trait JwtServiceTrait: Send + Sync {
    fn encode(&self, sub: String) -> JwtResult<String>;
    fn decode(&self, token: &str) -> JwtResult<Claims>;
}

pub struct JwtService {
    secret: String,
}
impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl JwtServiceTrait for JwtService {
    fn encode(&self, sub: String) -> JwtResult<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .expect("valid timestamp")
            .timestamp() as usize;
        let claims = Claims {
            sub,
            exp: expiration,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    fn decode(&self, token: &str) -> JwtResult<Claims> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

impl JwtServiceTrait for Arc<JwtService> {
    fn encode(&self, sub: String) -> JwtResult<String> {
        (**self).encode(sub)
    }
    fn decode(&self, token: &str) -> JwtResult<Claims> {
        (**self).decode(token)
    }
}
