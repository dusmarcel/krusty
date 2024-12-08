use actix_web::{get, web, Responder, Result};

use crate::Backend;

#[get("/back/users")]
async fn users(backend: web::Data<Backend>) -> Result<impl Responder> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&backend.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(count))
}