use std::sync::Mutex;

use actix_web::{get, web, Responder, Result};

use crate::Backend;

#[get("/users")]
async fn users(backend: web::Data<Mutex<Backend>>) -> Result<impl Responder> {
    let my_backend = backend.lock().unwrap();
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&my_backend.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(count))
}