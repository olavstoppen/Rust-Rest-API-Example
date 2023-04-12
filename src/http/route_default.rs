use actix_web::{HttpResponse, Responder};

use crate::models::error::ErrorResponse;

pub async fn default_route() -> impl Responder {
    let response = ErrorResponse {
        error: "Address not valid".to_string(),
        message: "Nice try, but you cannot do this!".to_string(),
        statuscode: 404,
    };

    HttpResponse::NotFound().json(response)
}
