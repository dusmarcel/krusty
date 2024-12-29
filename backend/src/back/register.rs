use std::sync::Mutex;

use actix_web::{error, get, post, web, Responder, Result};
use argon2::{Argon2, PasswordHasher};
use openssl::rsa::Rsa;
use password_hash::{rand_core::OsRng, SaltString};
use serde::Deserialize;
use uuid::Uuid;

use crate::back::Backend;

#[derive(Deserialize)]
struct FormData {
    username: String,
    email: String,
    password: String
}

#[post("/register")]
async fn register(backend: web::Data<Mutex<Backend>>, form: web::Form<FormData>) -> Result<impl Responder> {
    let mut my_backend = backend.lock().unwrap();
    if my_backend.registration_allowed {
        let id = sqlx::types::Uuid::from_u128(Uuid::now_v7().as_u128());
        if let Ok(rsa) = Rsa::generate(2048) {
            if let Ok(private_key_pem) = rsa.private_key_to_pem() {
                if let Ok(private_key) = String::from_utf8(private_key_pem) {
                    if let Ok(public_key_pem) = rsa.public_key_to_pem() {
                        if let Ok(public_key) = String::from_utf8(public_key_pem) {
                            let salt = SaltString::generate(&mut OsRng);
                            let argon2 = Argon2::default();
                            let hash = argon2.hash_password(form.password.as_bytes(), &salt).unwrap();
                            let result = sqlx::query(
                                    "INSERT INTO users (id, email, preferred_username, private_key, public_key, salt, hash) VALUES ($1, $2, $3, $4, $5, $6, $7)"
                                )
                                .bind(&id)
                                .bind(&form.email)
                                .bind(&form.username)
                                .bind(private_key)
                                .bind(public_key)
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
                            Ok(web::Redirect::to("/register").see_other())
                        }
                    } else {
                        Ok(web::Redirect::to("/register").see_other())
                    }
                } else {
                    Ok(web::Redirect::to("/register").see_other())
                }
            } else {
                Ok(web::Redirect::to("/register").see_other())
            }
        } else {
            Ok(web::Redirect::to("/register").see_other())
        }
    } else {
        Err(error::ErrorForbidden("Registration not allowed!"))
    }
}

#[get("/registration_allowed")]
async fn registration_allowed(backend: web::Data<Mutex<Backend>>) -> impl Responder {
    let my_backend = backend.lock().unwrap();
    web::Json(my_backend.registration_allowed)
}