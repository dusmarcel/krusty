include!("../valkey_config.rs");
include!("../backend_config.rs");
include!("../secret_key.rs");

use std::sync::Mutex;
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use env_logger::Env;

use backend::{
    back::{
        Backend,
        back,
        login::login,
        logout::logout,
        post::post,
        register::{register, registration_allowed},
        user::user as back_user,
        users::users
    },
    user::user,
    well_known::{
        host_meta::host_meta,
        webfinger::webfinger
    }
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(err, actix_web::HttpResponse::BadRequest().finish()).into()
        });

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
            .app_data(json_cfg.clone())
            .app_data(web::Data::clone(&data))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_path("/".to_string())
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::weeks(2))   
                    )
                    .build()
            )
            .service(
                web::scope("/back")
                    .service(back)
                    .service(login)
                    .service(logout)
                    .service(post)
                    .service(register)
                    .service(registration_allowed)
                    .service(back_user)
                    .service(users)
            )
            .service(
                web::scope("/.well-known")
                    .service(host_meta)
                    .service(webfinger)
            )
            .service(
                web::scope("/user")
                    .service(user)            )
    })
    .bind((BACKEND_IF, 8161))?
    .run()
    .await
}
