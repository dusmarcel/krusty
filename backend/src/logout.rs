use actix_session::Session;
use actix_web::{web, get, Responder};

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    web::Redirect::to("/").see_other()
}