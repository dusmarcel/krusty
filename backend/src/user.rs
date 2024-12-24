use std::sync::Mutex;

use actix_web::{error, get, web, Responder, Result};
use actix_session::Session;
use uuid::Uuid;

use crate::{Backend, actor::Actor};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub preferred_username: String,
    pub summary: Option<String>,
    pub private_key: String,
    pub public_key: String,
    pub salt: String,
    pub hash: String
}

#[get("/user/{user}")]
async fn user(backend: web::Data<Mutex<Backend>>, path: web::Path<String>) -> Result<impl Responder> {
    let my_backend = backend.lock().unwrap();
    let user = path.into_inner();
    let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE preferred_username = $1"
        )
        .bind(&user)
        .fetch_optional(&my_backend.pool)
        .await;

    match result {
        Ok(res) => {
            match res {
                Some(r) => {
                    if let Some(host) = &my_backend.host {
                        let actor = Actor::new(&host, &r);
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

impl User {
    pub fn to_shared(&self) -> shared::user::User {
        shared::user::User {
            id: self.id,
            email: self.email.clone(),
            name: self.name.clone(),
            preferred_username: self.preferred_username.clone(),
            summary: self.summary.clone(),
            public_key: self.public_key.clone()
        }
    }
}

#[get("/back/user")]
async fn b_user(backend: web::Data<Mutex<Backend>>, session: Session) -> Option<impl Responder> {
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

                match result {
                    Ok(res) => {
                        match res {
                            Some(r) => Some(web::Json(r.to_shared())),
                            None => None,
                        }
                    }                  
                    Err(e) => {
                        eprintln!("Error while executing database query: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}