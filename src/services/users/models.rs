use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AllUsers {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}