use actix_web::{get, web, Responder, Result};

use crate::{
    Backend,
    user::User
};

#[get("/users")]
async fn users(backend: web::Data<Backend>) -> Result<impl Responder> {
    let users_vec = sqlx::query_as::<_ , User>("SELECT * FROM users")
        .fetch_all(&backend.pool).await.unwrap();

    Ok(web::Json(users_vec.len()))
}