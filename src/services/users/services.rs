use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use super::models::{AllUsers, RegisterUser,UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash};

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
        Err(_e) => { HttpResponse::InternalServerError().body(format!("Erro ao tentar buscar um novo usuário: {}", _e)) }
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
        Err(e) => { HttpResponse::InternalServerError().body(format!("Erro ao tentar criar um novo usuário: {}", e)) }
    }
}
#[put("/users/{id}")]
async fn update_user(app_state: web::Data<AppState>, user: web::Json<UpdateUser>, id: web::Path<i32>) -> impl Responder {
    let hashed = hash(user.password.clone(), DEFAULT_COST).expect("Erro ao criptografar a senha");
    
    if !(hashed != user.password){
        return HttpResponse::InternalServerError().body("Erro ao tentar criptografar a senha!");
    }

    let result = sqlx::query!(
        "UPDATE users SET username = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, username, email, password", 
        user.username,
        user.email,
        hashed,
        id.into_inner()
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
        Err(e) => { HttpResponse::InternalServerError().body(format!("Erro ao tentar atualizar um usuário: {}", e)) }
}}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1 RETURNING id, username, email, password",
        id.into_inner()
    )
    .fetch_optional(&app_state.postgres_client)
    .await;
    
    match result {
        Ok(_) =>  HttpResponse::Ok().body(format!("User deletado com sucesso")),
        Err(_e) => {HttpResponse::InternalServerError().body(format!("Erro ao tentar deletar um usuário: {}", _e)) }
    }
}
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}