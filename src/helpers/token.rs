use crate::models::users::{Tokens, TokenClaims};
use crate::config::Config;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn refresh_tokens(user_id: &str, config: &Config) -> Tokens {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let access_token_exp = (now + Duration::hours(24)).timestamp() as usize;
    let refresh_token_exp = (now + Duration::days(7)).timestamp() as usize;

    let access_token_claims = TokenClaims {
        sub: user_id.to_string(),
        exp: access_token_exp,
        iat,
        refresh_token: uuid::Uuid::new_v4().to_string(),
    };
    let access_token = encode(
        &Header::default(),
        &access_token_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    let refresh_token_claims = TokenClaims {
        sub: user_id.to_string(),
        exp: refresh_token_exp,
        iat,
        refresh_token: uuid::Uuid::new_v4().to_string(),
    };
    let refresh_token = encode(
        &Header::default(),
        &refresh_token_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    Tokens::new(access_token, refresh_token)

}
