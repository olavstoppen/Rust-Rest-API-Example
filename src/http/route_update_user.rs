use actix_multipart::Multipart;
use actix_web::{http::header, post, web, HttpResponse, Responder};
use futures::StreamExt;
use futures::TryStreamExt;
use serde_json::from_str;
use serde_json::json;
use sled::Db;
use std::error::Error;
use std::str;

use crate::config::Config;
use crate::helpers::{image, jwt_auth, token};
use crate::models::error::ErrorResponse;
use crate::models::users::UpdateUserResponse;
use crate::models::users::{UpdateUserRequest, User, UserResponse};

#[post("/update_user")]
pub async fn update_user(
    config: web::Data<Config>,
    db: web::Data<Db>,
    mut payload: Multipart,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    // we need get the user data from the body
    let mut user_profile: Option<UpdateUserRequest> = None;
    let mut image_field: Option<Vec<u8>> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        if let Some(name) = content_disposition.get_name() {
            match name {
                "user_profile" => {
                    let mut bytes = web::BytesMut::new();
                    while let Some(chunk) = field.next().await {
                        bytes.extend_from_slice(&chunk.unwrap());
                    }
                    let json_str = str::from_utf8(&bytes).unwrap();
                    user_profile = Some(from_str::<UpdateUserRequest>(json_str).unwrap());
                }
                "image" => {
                    let mut content = Vec::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        content.extend_from_slice(&data);
                    }

                    image_field = Some(content);
                }
                _ => {}
            }
        }
    }

    let update_user_request = match user_profile {
        Some(user_data_from_request) => user_data_from_request,
        None => return HttpResponse::InternalServerError().json(json!("Error updating user")),
    };

    match update_user_in_db(&config, &db, update_user_request, image_field).await {
        Ok(updated_user) => {
            // The user updated, we update the tokens too
            let user_id = uuid::Uuid::parse_str(&updated_user.user_id).unwrap().to_string();
            let tokens = token::refresh_tokens(&user_id, config.get_ref());
            let response = json!(UpdateUserResponse::new("User Updated".to_string(), updated_user));

            HttpResponse::Ok()
                .append_header(("Refresh-Token", tokens.refresh_token))
                .append_header((header::AUTHORIZATION, format!("Bearer {}", tokens.access_token)))
                .json(response)
        }
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse::new("User update error!".to_string(), "User update error!".to_string(), 401)),        
    }
}

async fn update_user_in_db(
    config: &Config,
    db: &Db,
    update_user_request: UpdateUserRequest,
    imf: Option<Vec<u8>>,
) -> Result<UserResponse, Box<dyn Error>> {
    let user_table = db.open_tree(config.user_table.as_str())?;
    let mut new_image_generate = false;

    // Deserialize the existing user, update the fields, and serialize it back
    if let Ok(Some(user_bytes)) = user_table.get(update_user_request.user_id.as_bytes()) {
        let mut user: User = serde_json::from_slice(&user_bytes)?;
        if let Some(username) = update_user_request.username {

            if username != user.username {
                // if user changed the username we add to the index and remove the old username from the index
                let username_index_table = db.open_tree(&config.username_index_table).unwrap();
                username_index_table.insert(username.as_bytes(), user.user_id.as_bytes()).unwrap();
                username_index_table.remove(user.username.as_bytes()).unwrap(); 
            }

            user.username = username;
        }

        if let Some(email) = update_user_request.email {

            if email != user.email {
                // if user changed the email address we add to the index and remove the old email address index
                let email_index_table = db.open_tree(&config.email_index_table).unwrap();
                email_index_table.insert(email.as_bytes(), user.user_id.as_bytes()).unwrap();
                email_index_table.remove(user.email.as_bytes()).unwrap(); 
            }

            user.email = email;
        }

        if let Some(first_name) = update_user_request.first_name {
            user.first_name = first_name;
        }

        if let Some(last_name) = update_user_request.last_name {
            user.last_name = last_name;
        }

        if let Some(ud) = update_user_request.user_data {
            user.user_data = ud
        }

        let image_path = if let Some(field) = imf {
            if !field.is_empty() {
                new_image_generate = true;
                image::save_image(config, field, 500).await
            } else {
                user.profile_image_url
            }
        } else {
            user.profile_image_url
        };

        if !image_path.contains("default") && new_image_generate {
            if let Some(profile_image_orig) = &update_user_request.profile_image_url {
                let oldpath = profile_image_orig.replace(config.url_images.as_str(), "images/");
                let _ = image::delete_profile_image(&oldpath);
            }
        }

        user.profile_image_url = image_path;

        let user_bytes = serde_json::to_vec(&user)?;
        user_table.insert(update_user_request.user_id.as_bytes(), user_bytes)?;

        let resp = UserResponse::new(
            user.user_id, 
            user.username, 
            user.first_name, 
            user.last_name, 
            user.salt, 
            user.email,
            user.profile_image_url,
            user.user_data);

        Ok(resp)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "User not found",
        )))
    }
}
