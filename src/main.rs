use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };
use dotenv::dotenv;
use sqlx::{ Pool, Postgres };
mod databases {
    pub mod postgres_connection;
}

mod services;
#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = databases::postgres_connection::start_connection().await;
    let ip: String = std::env::var("IP").unwrap_or("127.0.0.1".to_string());
    let port: u16 = std::env
        ::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Failed to parse port");

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    postgres_client: _pool.clone(),
                })
            )
            .service(index)
            .configure(services::users::services::user_routes)
    })
        .bind((ip, port))?
        .run().await
}
