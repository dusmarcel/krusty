use actix_web::{error, get, web, Responder, Result};

use crate::Backend;

#[derive(sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String,
    salt: String,
    hash: String
}

#[get("/user/{user}")]
async fn user(backend: web::Data<Backend>, path: web::Path<String>) -> Result<impl Responder> {
    let user = path.into_inner();
    if let Some(b_user) = &backend.user {
        if *b_user == user {
            Ok(web::Json(backend.actor.to_shared()))
        } else {
            Err(error::ErrorNotFound("Not found!"))
        }
    } else {
        Err(error::ErrorInternalServerError("Internal server error!"))
    }
}