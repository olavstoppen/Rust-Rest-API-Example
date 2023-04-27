use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;

mod http;
mod config;
mod models;
mod db;
mod helpers;

use crate::http::{
    route_check_username, route_default, route_delete_user, route_get_user, route_get_users,
    route_login, route_refresh_token, route_save_user, route_update_user, route_check_email
};

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    let config = Config::init();

    let db_users = match db::database::init_sled_user_db() {
        Ok(db_users) => db_users,
        Err(e) => {
            eprintln!("Failed to initialize user database: {e}");
            // Exit if user DB has error
            std::process::exit(1);
        }
    };
    
    //insert_dummy_users(&db_users.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            // APP DATA
            .app_data(web::Data::new(db_users.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .default_service(
                web::route()
                    // Any 404 request we pass a default error in jSON
                    .guard(actix_web::guard::Get())
                    //.guard(actix_web::guard::Post())
                    .guard(actix_web::guard::Delete())
                    .to(route_default::default_route),
            )
            .configure(app_config)
    })
    .bind("127.0.0.1:8484")?
    .run()
    .await
}

fn app_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(route_get_user::get_user)
        .service(route_get_users::get_all_users)
        .service(route_check_username::check_username)
        .service(route_update_user::update_user)
        .service(route_save_user::save_user)
        .service(route_delete_user::delete_user)
        .service(route_login::login_user_handler)
        .service(route_refresh_token::refresh_token_handler)
        .service(route_check_email::check_email);

    conf.service(scope);
}