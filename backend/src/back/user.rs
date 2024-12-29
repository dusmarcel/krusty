use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use actix_session::Session;
use uuid::Uuid;

use crate::{back::Backend, user::User};

#[get("/user")]
async fn user(backend: web::Data<Mutex<Backend>>, session: Session) -> impl Responder {
    let my_backend = backend.lock().unwrap();
    if let Ok(id) =  session.get::<String>("id") {
        if let Some(id) = id {
            if let Ok(id) = Uuid::parse_str(&id) {
                let result = sqlx::query_as::<_, User>(
                        "SELECT * FROM users WHERE id = $1"
                    )
                    .bind(&id)
                    .fetch_optional(&my_backend.pool)
                    .await;

                if let Ok(Some(u)) = result {
                    return HttpResponse::Ok().json(u.to_shared())
                }
            }
        }
    }
    HttpResponse::Ok().body("")
}