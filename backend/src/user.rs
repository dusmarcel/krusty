use std::sync::Mutex;

use actix_web::{error, get, web, Responder, Result};
use uuid::Uuid;

use crate::{Backend, actor::Actor};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub salt: String,
    pub hash: String
}

#[get("/user/{user}")]
async fn user(backend: web::Data<Mutex<Backend>>, path: web::Path<String>) -> Result<impl Responder> {
    let my_backend = backend.lock().unwrap();
    let user = path.into_inner();
    let result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE name = $1"
    )
    .bind(&user)
    .fetch_optional(&my_backend.pool)
    .await;

    match result {
        Ok(res) => {
            match res {
                Some(r) => {
                    if let Ok(actor) = Actor::new(&my_backend.host, &Some(r.name)) {
                        Ok(web::Json(actor.to_shared()))
                    } else {
                        Err(error::ErrorInternalServerError("Internal server error!"))
                    }
                },
                None => {
                    Err(error::ErrorNotFound("Not found!"))
                }
            }
        }                  
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(error::ErrorInternalServerError("Internal server error!"))
        }
    }
}