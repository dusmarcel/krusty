include!("../postgres_config.rs");

use std::{env, sync::Mutex};

use anyhow::Result;
use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, web};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod actor;
mod key;
pub mod login;
pub mod logout;
pub mod register;
pub mod user;
pub mod users;
//pub mod webfinger;
pub mod well_known;
pub mod link;

use user::User;
use uuid::Uuid;

#[derive(Clone)]
pub struct Backend {
    pub host: Option<String>,
    pub pool: Pool<Postgres>,
    pub registration_allowed: bool
}

impl Backend {
    pub async fn new () -> Result<Self> {
        let mut host = None;
        if let Ok(env_host) = env::var("HOST") {
            host = Some(env_host.clone());
        }
 
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(POSTGRES_URL)
            .await?;

        sqlx::migrate!()
            .run(&pool)
            .await?;

        let registration_allowed;
        // The default behaviour is, that only one regstration shall be allowed
        // i. e., if no user exists yet
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await?;

        if count == 0 {
            registration_allowed = true;
        } else {
            registration_allowed = false;
        }

        Ok(Self {
            host,
            pool,
            registration_allowed
        })
    }
}

#[get("/")]
async fn back(backend: web::Data<Mutex<Backend>>, session: Session) -> impl Responder {
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
                        HttpResponse::Ok().body(format!("Hello, {}!", res.unwrap().preferred_username))
                    }
                    Err(e) => {
                        eprintln!("Error while executing query: {}", e);
                        HttpResponse::Ok().body("Hello from Krusty!")
                    }
                }
            } else {
                HttpResponse::Ok().body("Hello from Krusty!")
            }
        } else {
            HttpResponse::Ok().body("Hello from Krusty!")
        }
    } else {
        HttpResponse::Ok().body("Hello from Krusty!")
    }
}
