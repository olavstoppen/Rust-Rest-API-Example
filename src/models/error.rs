use core::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub statuscode: i32,
}

impl ErrorResponse {
    pub fn new(error: String, message: String, statuscode: i32 ) -> Self {
        ErrorResponse { error, message, statuscode }
    }
}

impl fmt::Display for ErrorResponse {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
