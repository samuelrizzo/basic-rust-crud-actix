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
                    name: user.username.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
                .collect::<Vec<AllUsers>>()
        ) }
        Err() => { HttpResponse::InternalServerError().body("Erro ao tentar buscar os usuários!") }
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
}