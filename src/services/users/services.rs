use std::future::IntoFuture;

use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use super::models::{AllUsers, RegisterUser,UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash, verify};

#[get("/users")]
async fn get_all_users (app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&app_state.postgres_client)
        .await;
    
    match result {
        Ok(users) => { HttpResponse::Ok().json(
            users
                .iter()
                .map(|user| AllUsers {
                    id: user.id,
                    username: user.username.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
                .collect::<Vec<AllUsers>>()
        ) }
        Err(e) => { HttpResponse::InternalServerError().body("Erro ao tentar buscar os usuários!") }
    }
}
#[post("/users")]
async fn create_user(app_state: web::Data<AppState>, user: web::Json<RegisterUser>) -> impl Responder {
    let hashed = hash(user.password.clone(), DEFAULT_COST).expect("Erro ao criptografar a senha");
    
    if !(hashed != user.password){
        return HttpResponse::InternalServerError().body("Erro ao tentar criptografar a senha!");
    }

    let result = sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email, password", 
        user.username,
        user.email,
        hashed
    )
    .fetch_one(&app_state.postgres_client)
    .await;
    
    match result {
        Ok(user) => { HttpResponse::Ok().json(AllUsers {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        }) }
        Err(_) => { HttpResponse::InternalServerError().body("Erro ao tentar criar um novo usuário!") }
}}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_user);
}