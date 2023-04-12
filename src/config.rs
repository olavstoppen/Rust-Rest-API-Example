#[derive(Debug, Clone)]
pub struct Config {
    pub user_table: String,
    pub username_index_table: String,
    pub email_index_table: String,
    pub user_data_table: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub url_images: String,
    pub url_default_images: String,
}

impl Config {
    pub fn init() -> Config {
        let user_table = std::env::var("USER_TABLE").expect("USER_TABLE must be set");
        let username_index_table = std::env::var("USERNAME_INDEX").expect("USERNAME_INDEX must be set");
        let email_index_table = std::env::var("EMAIL_INDEX").expect("EMAIL_INDEX must be set");
        let user_data_table =
            std::env::var("USER_DATA_TABLE").expect("USER_DATA_TABLE must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let url_images = std::env::var("URL_IMAGES").expect("URL_MAGES must be set");
        let url_default_images = std::env::var("URL_IMAGES").expect("URL_MAGES must be set");

        Config {
            user_table,
            username_index_table,
            email_index_table,
            user_data_table,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            url_images,
            url_default_images,
        }
    }

}
