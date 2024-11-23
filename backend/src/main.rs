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
    .bind(("127.0.0.1", 8161))?
    .run()
    .await
}
