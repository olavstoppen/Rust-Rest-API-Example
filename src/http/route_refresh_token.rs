use actix_web::{get, http, http::header, http::Error, web, HttpRequest, HttpResponse};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::config::Config;
use crate::models::error::ErrorResponse;
use crate::models::users::TokenClaims;
use serde_json::json;

#[get("/refresh_token")]
pub async fn refresh_token_handler(
    config: web::Data<Config>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let refresh_token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|h| h.split_whitespace().last().unwrap().to_string());

    if refresh_token.is_none() {
        let json_error = ErrorResponse {
                error: "Unauthorized".to_string(),
                message: "Refresh token is missing. Please include a valid refresh token in the Authorization header of your request.".to_string(),
                statuscode: 401
            };
       
        return Ok(HttpResponse::Unauthorized().json(json_error));
    }

    let decoded_refresh_token = match decode::<TokenClaims>(
        &refresh_token.unwrap(),
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => token,
        Err(_) => {
            let json_error = ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "Refresh token is invalid. Please include a valid refresh token in the Authorization header of your request.".to_string(),
                    statuscode: 401
                };
            
            return Ok(HttpResponse::Unauthorized().json(json_error));
        }
    };

    let user_id = uuid::Uuid::parse_str(decoded_refresh_token.claims.sub.as_str()).unwrap();

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
    
    Ok(HttpResponse::Ok()
        .append_header(("Refresh-Token", refresh_token))
        .append_header((header::AUTHORIZATION, format!("Bearer {}", access_token)))
        .json(json!({"success:": true,  "message": "Done" })))
}
