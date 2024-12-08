use actix_web::{post, web, Responder};
use argon2::{Argon2, PasswordHasher};
use password_hash::{rand_core::OsRng, SaltString};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[post("/back/register")]
async fn register(form: web::Form<FormData>) -> impl Responder {
    println!("username: {}", form.username);
    println!("password: {}", form.password);

    let salt = SaltString::generate(&mut OsRng);
    println!("salt: {}", salt);

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(form.password.as_bytes(), &salt).unwrap();
    println!("{}", hash);

    web::Redirect::to("/register").see_other()
}