use actix_web::{ get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod databases {
    pub mod postgres_connection;
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let _pool = databases::postgres_connection::start_connection().await;
    HttpServer::new(||{
        App::new()
        .service(index)
    }).bind(("127.0.0.1", 8080))?.run().await
}
