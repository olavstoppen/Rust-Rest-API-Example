use actix_web::{get, web, HttpResponse, Responder};
use sled::Db;
use std::error::Error;

use crate::config::Config;
use crate::models::users::User;
use crate::models::users::Users;

#[get("/get_all_users")]
pub async fn get_all_users(
    db: web::Data<Db>,
    config: web::Data<Config>
) -> impl Responder {
    match fetch_users_from_db(&config, &db) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching users"),
    }
}

fn fetch_users_from_db(
    config: &Config,
    db: &sled::Db
) -> Result<Users, Box<dyn Error>> {
    let user_table = db.open_tree(config.user_table.as_str())?;
    let mut users = Vec::new();

   // Calculate the start and end indices of the current page
   let start_index = 1 * 20;
   let end_index = (1 + 1) * 20;

   // Iterate over the entries in the user table and collect the current page of users
   let mut current_index = 0;
   for entry in user_table.iter() {
       if current_index >= start_index && current_index < end_index {
           let (_, user_bytes) = entry?;
           let user: User = serde_json::from_slice(&user_bytes)?;
           users.push(user);
       } else if current_index >= end_index {
           break;
       }

       current_index += 1;
   }

   let result_users = Users {
       user_count: users.len() as i32,
       users: users,
   };

    Ok(result_users)
}
