use actix_multipart::Multipart;
use actix_web::{http::header, post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use futures::StreamExt;
use futures::TryStreamExt;
use serde_json::from_str;
use serde_json::json;
use sled::Db;
use std::error::Error;
use std::str;
use uuid::Uuid;


use crate::config::Config;
use crate::helpers::{image, token};
use crate::models::users::{NewUser, UpdateUserResponse, User, UserResponse};
use crate::models::{error::ErrorResponse, users::UserData};

#[post("/signup")]
pub async fn save_user(
    config: web::Data<Config>,
    db: web::Data<Db>,
    mut payload: Multipart,
) -> impl Responder {
    // we need get the user data from the body

    let mut user_data: Option<NewUser> = None;
    let mut image_field: Option<Vec<u8>> = None;
    dbg!("save_user");

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();

        dbg!(&content_disposition.get_name());

        if let Some(name) = content_disposition.get_name() {
            dbg!(&name);

            match name {
                "userdata" => {
                    let mut bytes = web::BytesMut::new();
                    while let Some(chunk) = field.next().await {
                        bytes.extend_from_slice(&chunk.unwrap());
                    }
                    let json_str = str::from_utf8(&bytes).unwrap();
                    user_data = Some(from_str::<NewUser>(json_str).unwrap());
                }
                "image" => {
                    let mut content = Vec::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        content.extend_from_slice(&data);
                    }

                    image_field = Some(content);
                }
                _ => {
                    // other fields righzt now do nothing!
                }
            }
        }
    }

    let new_user_request = match user_data {
        Some(new_user_data) => new_user_data,
        None => {
            return HttpResponse::Conflict().json(ErrorResponse {
                error: "save_user_in_db produce error".to_string(),
                message: "save_user_in_db produce error".to_string(),
                statuscode: 100,
            })
        }
    };

    match save_user_in_db(&config, &db, new_user_request, image_field).await {
        Ok(new_user) => {

            let user_id = uuid::Uuid::parse_str(&new_user.user_id).unwrap();
            let tokens = token::refresh_tokens(&user_id.to_string(), config.get_ref());
            let response = json!(UpdateUserResponse::new("User saved".to_string(), new_user));

            HttpResponse::Ok()
                .append_header(("Refresh-Token", tokens.refresh_token))
                .append_header((header::AUTHORIZATION, format!("Bearer {}", tokens.access_token)))
                .json(response)
        }
        Err(e) => {
            if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                if io_error.kind() == std::io::ErrorKind::InvalidData {
                    HttpResponse::Conflict().json(ErrorResponse {
                        error: "save_user_in_db produce error".to_string(),
                        message: io_error.to_string(),
                        statuscode: 409,
                    })
                } else {
                    HttpResponse::InternalServerError().json("Error saving user")
                }
            } else {
                HttpResponse::InternalServerError().json("Error saving user")
            }
        }
    }
}

async fn save_user_in_db(
    config: &Config,
    db: &Db,
    new_user_request: NewUser,
    imf: Option<Vec<u8>>,
) -> Result<UserResponse, Box<dyn Error>> {
    let user_table = db.open_tree(config.user_table.as_str())?;
    let username_index_table = db.open_tree(&config.username_index_table).unwrap();
    let email_index_table = db.open_tree(&config.email_index_table).unwrap();

    let user_uuid = &Uuid::new_v4().to_string();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(new_user_request.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Check if the user_id already exists in the database
    if user_table.contains_key(user_uuid)? {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "User exists!",
        )));
    }

    let image_path = match imf {
        Some(field) => image::save_image(config, field, 500).await,
        None => config.url_default_images.to_owned(),
    };

    let user_data = UserData {
        instagram: "".to_string(),
        twitter: "".to_string(),
        tiktok: "".to_string(),
        facebook: "".to_string(),
        description: "".to_string(),
    };

    let new_user = User {
        user_id: user_uuid.to_owned(),
        username: new_user_request.username,
        first_name: new_user_request.first_name,
        last_name: new_user_request.last_name,
        email: new_user_request.email,
        password: password_hash,
        salt: salt.to_string(),
        profile_image_url: image_path,
        user_data,
    };

    let user_bytes = serde_json::to_vec(&new_user)?;
    user_table.insert(user_uuid.clone(), user_bytes)?;
    username_index_table.insert(new_user.username.as_bytes(), user_uuid.as_bytes()).unwrap();
    email_index_table.insert(new_user.email.as_bytes(), user_uuid.as_bytes()).unwrap();

    let resp = UserResponse::new(
        new_user.user_id, 
        new_user.username, 
        new_user.first_name, 
        new_user.last_name, 
        new_user.salt, 
        new_user.email,
        new_user.profile_image_url,
        new_user.user_data);
    
    Ok(resp)
}
