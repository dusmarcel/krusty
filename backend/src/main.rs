include!("../backend_config.rs");

use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder, Result};

use backend::Backend;

#[get("/back")]
async fn back() -> impl Responder {
    HttpResponse::Ok().body("Hello from Krusty!")
}

#[get("/user/{user}")]
async fn user(backend: web::Data<Backend>, path: web::Path<String>) -> Result<impl Responder> {
    let user = path.into_inner();
    if let Some(b_user) = &backend.user {
        if *b_user == user {
            Ok(web::Json(backend.actor.to_shared()))
        } else {
            Err(error::ErrorNotFound("User not found!"))
        }
    } else {
        Err(error::ErrorInternalServerError("No user found!"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = Backend::new().map_err(|e| {
        eprintln!("I cannot work under these conditions! Error while building backend: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("backend error: {}", e))        
    })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(backend.clone()))
            .service(back)
            .service(user)
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
