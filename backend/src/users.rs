use actix_web::{get, web, Responder, Result};

use crate::{
    Backend,
    user::User
};

#[get("/back/users")]
async fn users(backend: web::Data<Backend>) -> Result<impl Responder> {
    let users_vec = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&backend.pool)
        .await?;

    Ok(web::Json(users_vec.len()))
}