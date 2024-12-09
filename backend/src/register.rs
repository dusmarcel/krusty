use actix_web::{post, web, Responder};
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
async fn register(backend: web::Data<Backend>, form: web::Form<FormData>) -> impl Responder {
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

    match result {
        Ok(_) => web::Redirect::to("/login").see_other(),
        Err(e) => {
            eprintln!("Error inserting user: {}", e.to_string());
            web::Redirect::to("/register").see_other()
        }
    }
}