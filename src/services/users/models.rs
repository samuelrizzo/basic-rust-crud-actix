use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AllUsers {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}