use actix_web::{ get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde_json::Number;

mod databases {
    pub mod postgres_connection;
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
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Failed to parse port");
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind((ip, port))?
    .run()
    .await
}

