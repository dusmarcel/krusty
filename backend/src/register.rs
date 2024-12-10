use actix_web::{error, get, post, web, Responder, Result};
use argon2::{Argon2, PasswordHasher};
use password_hash::{rand_core::OsRng, SaltString};
use serde::Deserialize;

use crate::Backend;

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[post("/back/register")]
async fn register(backend: web::Data<Backend>, form: web::Form<FormData>) -> Result<impl Responder> {
    if backend.registration_allowed {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(form.password.as_bytes(), &salt).unwrap();
        let result = sqlx::query(
                "INSERT INTO users (id, name, salt, hash) VALUES (DEFAULT, $1, $2, $3)"
            )
            .bind(&form.username)
            .bind(&salt.to_string())
            .bind(&hash.to_string())
            .execute(&backend.pool)
            .await;

        // TBD: set registration allowed to false!

        match result {
            Ok(_) => Ok(web::Redirect::to("/login").see_other()),
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
async fn registration_allowed(backend: web::Data<Backend>) -> impl Responder {
    web::Json(backend.registration_allowed)
}