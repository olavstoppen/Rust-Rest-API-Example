use actix_web::{get, web, HttpResponse, Responder};
use sled::Db;

use crate::config::Config;
use crate::models::users::{CheckUsernameResponse};

#[get("/check_username/{user_name}")]
pub async fn check_username(
    db: web::Data<Db>,
    config: web::Data<Config>,
    username: web::Path<String>
) -> impl Responder {

    let resp = check_username_availability(&config, &db, &username);

    HttpResponse::Ok()
    .content_type("application/json")
    .json(resp)    

}

pub fn check_username_availability(
    config: &Config,
    db: &Db,
    username: &str,
) -> CheckUsernameResponse {
    let username_index_table = db.open_tree(&config.username_index_table).unwrap();
    let response = match username_index_table.get(username.as_bytes()) {
        Ok(Some(_)) => {
            CheckUsernameResponse::new(false, "Username not available".to_string())
        }
        Ok(None) => {
            CheckUsernameResponse::new(true, "Username available".to_string())
        }
        Err(_) => {
            CheckUsernameResponse::new(false, "Username not available".to_string())
        }
    };

    response
}
