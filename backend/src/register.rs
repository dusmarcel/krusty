use actix_web::{post, web, Responder};
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

    web::Redirect::to("/register").see_other()
}