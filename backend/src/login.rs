use actix_web::{post, web, Responder};

#[post("/back/login")]
async fn login() ->impl Responder {
    web::Redirect::to("/").see_other()
}