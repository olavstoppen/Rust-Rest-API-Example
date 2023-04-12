use actix_web::{http::header, post, web, HttpResponse, Responder};
use serde_json::from_slice;
use sled::Db;

use crate::config::Config;
use crate::models::error::ErrorResponse;
use crate::models::users::{LoginRequest, LoginResponse, User, UserResponse};
use crate::helpers::token;

#[post("/login")]
async fn login_user_handler(
    db: web::Data<Db>,
    body: web::Json<LoginRequest>,
    config: web::Data<Config>
) -> impl Responder {
    match find_user_by_email(&config, &db, &body.email).await {
        Some(user) => {
            if user.validate_password(&body.password) {
                
                let tokens = token::refresh_tokens(&user.user_id, config.get_ref());

                let usr = UserResponse::new(
                    user.user_id, 
                    user.username, 
                    user.first_name, 
                    user.last_name, 
                    user.salt, 
                    user.email, 
                    user.profile_image_url, 
                    user.user_data);

                let resp = LoginResponse {
                    success: true,
                    user_data: usr,
                };

                HttpResponse::Ok()
                    .append_header((header::AUTHORIZATION, format!("Bearer {}", tokens.access_token)))
                    .append_header(("Refresh-Token", tokens.refresh_token))
                    .json(resp)

            } else {

                let json_error = ErrorResponse {
                    error: "User email or password incorrect!".to_string(),
                    message: "User email or password incorrect!".to_string(),
                    statuscode: 404,
                };
                
                HttpResponse::Conflict().json(json_error)
            }
        }
        None => {
            let json_error = ErrorResponse::new("User email or password incorrect!".to_string(), "User email or password incorrect!".to_string(), 401);
            HttpResponse::Conflict().json(json_error)
        }
    }
}

async fn find_user_by_email(config: &Config, db: &Db, email: &str) -> Option<User> {
    let users_tree = db
        .open_tree(config.user_table.as_str())
        .expect("Failed to open 'users' tree");

    let mut found_user: Option<User> = None;

    for result in users_tree.iter() {
        let (_, user_data) = result.expect("Failed to read user data");
        let user: User = from_slice(&user_data).expect("Failed to deserialize user data");
        if user.email == email {
            found_user = Some(user);
            break;
        }
    }

    found_user
}
