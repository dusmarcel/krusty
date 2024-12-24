include!("../valkey_config.rs");
include!("../backend_config.rs");
include!("../secret_key.rs");

use std::sync::Mutex;
use actix_web::{cookie::Key, web, App, HttpServer};
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};

use backend::{
    Backend,
    back,
    login::login,
    logout::logout,
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

    let secret_key = Key::from(SECRET_KEY.as_bytes());
    let redis_store = RedisSessionStore::new(VALKEY_URL)
        .await
        .map_err(|e| {
            eprintln!("Could'nt create valkey/redis store! Error message: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, format!("valkey/redis error: {}", e))
        })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&data))
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_path("/".to_string())
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::weeks(2))   
                    )
                    .build()
            )
            .service(back)
            .service(login)
            .service(logout)
            .service(register)
            .service(registration_allowed)
            .service(user)
            .service(b_user)
            .service(users)
            .service(webfinger)
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
