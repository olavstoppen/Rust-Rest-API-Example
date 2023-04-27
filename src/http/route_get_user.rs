use actix_web::{get, web, HttpResponse, Responder};
use sled::Db;
use std::error::Error;

use crate::config::Config;
use crate::helpers::jwt_auth;
use crate::models::error::ErrorResponse;
use crate::models::users::{User, UserResponse};

#[get("/get_user/{user_id}")]
pub async fn get_user(
    db: web::Data<Db>,
    config: web::Data<Config>,
    user_id: web::Path<String>,
    _: jwt_auth::JwtMiddleware
) -> impl Responder {
    match fetch_user_from_db(&config, &db, &user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "fetch_user_from_db function produce error".to_string(),
            message: "Error happened in get User".to_string(),
            statuscode: 500,
        }),
    }
}

fn fetch_user_from_db(
    config: &Config,
    db: &Db,
    user_id: &str
) -> Result<UserResponse, Box<dyn Error>> {
    let user_table = db.open_tree(config.user_table.as_str())?;

    let key = user_id.to_string();
    let user_bytes = user_table.get(key)?;

    match user_bytes {
        Some(data) => {
            let user: User = serde_json::from_slice(&data)?;
            let resp = UserResponse {
                username: user.username,
                user_id: user.user_id,
                first_name: user.first_name,
                last_name: user.last_name,
                salt: user.salt,
                email: user.email,
                profile_image_url: user.profile_image_url,
                user_data: user.user_data,
            };

            Ok(resp)
        }
        None => {
            Err("User not found".into())
        }
    }
}
