use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}