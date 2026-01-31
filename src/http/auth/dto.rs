use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub id: Uuid,
    pub email: String,
}

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}