use actix_web::{post, web, Responder};
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

    web::Redirect::to("/register").see_other()
}