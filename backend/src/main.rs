include!("../valkey_config.rs");
include!("../backend_config.rs");

use std::sync::Mutex;

use actix_web::{cookie::Key, web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};

use backend::{
    Backend,
    back,
    login::login,
    register::{register, registration_allowed},
    user::user,
    users::users,
    webfinger::webfinger
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = Backend::new()
        .await
        .map_err(|e| {
            eprintln!("I cannot work under these conditions! Error while building backend: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, format!("backend error: {}", e))        
        })?;

    let data = web::Data::new(Mutex::new(backend));

    println!("VALKEY_URL={}", VALKEY_URL);
    // let secret_key = Key::generate();
    // let redis_store = RedisSessionStore::new(VALKEY_URL)
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Could'nt create valkey/redis store! Error message: {}", e);
    //         std::io::Error::new(std::io::ErrorKind::Other, format!("valkey/redis rror: {}", e))
    //     })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&data))
            // .wrap(
            //     SessionMiddleware::new(
            //         redis_store.clone(),
            //         secret_key.clone()
            //     )
            // )
            .service(back)
            .service(login)
            .service(register)
            .service(registration_allowed)
            .service(user)
            .service(users)
            .service(webfinger)
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
