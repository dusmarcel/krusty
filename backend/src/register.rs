use actix_web::{post, web::Redirect, Responder};

#[post("/back/register")]
async fn register() -> impl Responder {
    Redirect::to("/register").see_other()
}