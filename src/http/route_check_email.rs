use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use sled::Db;

use crate::{config::Config};
use crate::models::users::{CheckEmailResponse};

#[get("/check_email/{email_address}")]
pub async fn check_email(
    db: web::Data<Db>,
    config: web::Data<Config>,
    email: web::Path<String>
) -> impl Responder {
    let resp = check_email_availability(&config, &db, &email);
    HttpResponse::Ok()
                .content_type("application/json")
                .json(json!(resp))
}

pub fn check_email_availability(
    config: &Config,
    db: &Db,
    email: &str,
) -> CheckEmailResponse {

    let email_index_table = db.open_tree(&config.email_index_table).unwrap();

    let response = match email_index_table.get(email.as_bytes()) {
        Ok(Some(_)) => {
            CheckEmailResponse::new(false, "Email not available".to_string())
        }
        Ok(None) => {
            CheckEmailResponse::new(true, "Email available".to_string())
        }
        Err(_) => {
            CheckEmailResponse::new(false, "Email not available".to_string())
        }
    };

    response
    
}
