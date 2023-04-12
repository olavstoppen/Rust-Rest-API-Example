use actix_web::{delete, web, HttpResponse, Responder};
use serde_json::json;
use sled::Db;
use std::error::Error;

use crate::config::Config;
use crate::helpers::jwt_auth;

#[delete("/get_all_users")]
pub async fn delete_user(
    db: web::Data<Db>,
    config: web::Data<Config>,
    user_id: web::Path<String>,
    _: jwt_auth::JwtMiddleware   
) -> impl Responder {
    match delete_user_from_db(&config, &db, &user_id) {
        Ok(response) => HttpResponse::Ok()
            .content_type("application/json")
            .body(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn delete_user_from_db(
    config: &Config,
    db: &Db,
    user_id: &str
) -> Result<String, Box<dyn Error>> {
    let user_table = db.open_tree(config.user_table.as_str())?;

    if !user_table.contains_key(user_id)? {
        let response = json!({
            "success": false,
            "message": "User not found in the database!"
        });

        return Ok(response.to_string());
    }

    user_table.remove(user_id)?;

    let response = json!({
        "success": true,
        "message": "User deleted"
    });
    
    Ok(response.to_string())
}
