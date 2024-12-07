include!("../backend_config.rs");

use actix_web::{web, App, HttpServer};

use backend::{
    Backend,
    back,
    login,
    register::register,
    user::user,
    users::users,
    webfinger::webfinger
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = Backend::new().await.map_err(|e| {
        eprintln!("I cannot work under these conditions! Error while building backend: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("backend error: {}", e))        
    })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(backend.clone()))
            .service(back)
            .service(login)
            .service(register)
            .service(user)
            .service(users)
            .service(webfinger)
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
