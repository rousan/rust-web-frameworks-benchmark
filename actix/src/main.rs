use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index(info: web::Path<()>) -> impl Responder {
    format!("Hello, World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
