include!("../backend_config.rs");
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/back")]
async fn back() -> impl Responder {
    HttpResponse::Ok().body("Hello, Krusty!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(back)
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
