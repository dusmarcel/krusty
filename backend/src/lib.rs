include!("../database_config.rs");

use std::env;

use anyhow::Result;
use actix_web::{get, post, web::Redirect, HttpResponse, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod actor;
mod key;
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

        Ok(Self {
            host,
            user,
            actor,
            pool,
        })
    }
}

#[get("/back")]
async fn back() -> impl Responder {
    HttpResponse::Ok().body("Hello from Krusty!")
}

#[post("/back/login")]
async fn login() ->impl Responder {
    Redirect::to("/").see_other()
}