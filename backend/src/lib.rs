include!("../database_config.rs");

use std::env;

use anyhow::Result;
use actix_web::{get, HttpResponse, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod actor;
mod key;
pub mod login;
pub mod register;
pub mod user;
pub mod users;
pub mod webfinger;
pub mod link;

use crate::actor::Actor;

#[derive(Clone)]
pub struct Backend {
    pub host: Option<String>,
    pub user: Option<String>,
    pub actor: Actor,
    pub pool: Pool<Postgres>,
    pub registration_allowed: bool,
}

impl Backend {
    pub async fn new () -> Result<Self> {
        let mut host = None;
        if let Ok(env_host) = env::var("HOST") {
            host = Some(env_host.clone());
        }
        let mut user = None;
        if let Ok(env_user) = env::var("USER") {
            user = Some(env_user.clone())
        }
        let actor = Actor::new(&host, &user)?;

        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(DATABASE_URL)
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
            user,
            actor,
            pool,
            registration_allowed
        })
    }
}

#[get("/back")]
async fn back() -> impl Responder {
    HttpResponse::Ok().body("Hello from Krusty!")
}
