use argon2::{self, Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};

// This is the complete User in database
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub salt: String,
    pub profile_image_url: String,
    pub user_data: UserData,
}

impl User {
    pub fn validate_password(&self, password: &str) -> bool {
        let parsed_hash =
            PasswordHash::new(&self.password).expect("Failed to parse stored password hash");
        let argon2 = Argon2::default();

        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[derive()]
pub struct UserData {
    pub instagram: String,
    pub twitter: String,
    pub tiktok: String,
    pub facebook: String,
    pub description: String,
}

// When we response user we send back UserResponse
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub salt: String,
    pub email: String,
    pub profile_image_url: String,
    pub user_data: UserData,
}

impl UserResponse {
    pub fn new(user_id: String, username: String, first_name: String, last_name: String, salt: String, email: String, profile_image_url: String, user_data: UserData ) -> UserResponse {
        UserResponse { user_id, username, first_name, last_name, salt, email, profile_image_url, user_data }
    }
}

// create new User Request
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Request for update user
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user_id: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub image_base64: Option<String>,
    pub profile_image_url: Option<String>,
    pub user_data: Option<UserData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserResponse {
    message: String,
    user: UserResponse
}

impl UpdateUserResponse {
    pub fn new(message: String, user: UserResponse) -> Self {
        UpdateUserResponse { message , user }
    }
}

// Debigh log for user list
#[derive(Serialize, Deserialize)]
pub struct Users {
    pub user_count: i32,
    pub users: Vec<User>,
}

// Login request
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub user_data: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String
}

impl Tokens {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Tokens { access_token, refresh_token}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckEmailResponse {
    pub status: bool,
    pub message: String
}

impl CheckEmailResponse {
    pub fn new(status: bool, message: String) -> Self {
        CheckEmailResponse { status, message }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUsernameResponse {
    pub status: bool,
    pub message: String
}

impl CheckUsernameResponse {
    pub fn new(status: bool, message: String) -> Self {
        CheckUsernameResponse { status, message }
    }
}



