use std::sync::Mutex;

use actix_web::{error, get, post, web, Responder, Result};
use argon2::{Argon2, PasswordHasher};
use password_hash::{rand_core::OsRng, SaltString};
use serde::Deserialize;
use uuid::Uuid;

use crate::Backend;

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[post("/back/register")]
async fn register(backend: web::Data<Mutex<Backend>>, form: web::Form<FormData>) -> Result<impl Responder> {
    let mut my_backend = backend.lock().unwrap();
    if my_backend.registration_allowed {
        let uuid = Uuid::now_v7();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(form.password.as_bytes(), &salt).unwrap();
        let result = sqlx::query(
                "INSERT INTO users (uuid, name, salt, hash) VALUES ($1, $2, $3, $4)"
            )
            .bind(&uuid.to_string())
            .bind(&form.username)
            .bind(&salt.to_string())
            .bind(&hash.to_string())
            .execute(&my_backend.pool)
            .await;

        match result {
            Ok(_) => {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
                    .fetch_one(&my_backend.pool)
                    .await
                    .map_err(actix_web::error::ErrorInternalServerError)?;
                if count == 1 {
                    my_backend.registration_allowed = false;
                }
                Ok(web::Redirect::to("/login").see_other())
            }
            Err(e) => {
                eprintln!("Error inserting user: {}", e.to_string());
                Ok(web::Redirect::to("/register").see_other())
            }
        }
    } else {
        Err(error::ErrorForbidden("Registration not allowed!"))
    }
}

#[get("/back/registration_allowed")]
async fn registration_allowed(backend: web::Data<Mutex<Backend>>) -> impl Responder {
    let my_backend = backend.lock().unwrap();
    web::Json(my_backend.registration_allowed)
}