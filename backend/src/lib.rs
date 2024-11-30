use std::env;

use anyhow::Result;
use actix_web::{get, post, web::Redirect, HttpResponse, Responder};

mod actor;
mod key;
pub mod user;
pub mod webfinger;
pub mod link;

use crate::actor::Actor;

#[derive(Clone)]
pub struct Backend {
    pub host: Option<String>,
    pub user: Option<String>,
    pub actor: Actor,
}

impl Backend {
    pub fn new () -> Result<Self> {
        let mut host = None;
        if let Ok(env_host) = env::var("HOST") {
            host = Some(env_host.clone());
        }
        let mut user = None;
        if let Ok(env_user) = env::var("USER") {
            user = Some(env_user.clone())
        }
        let actor = Actor::new(&host, &user)?;

        Ok(Self {
            host,
            user,
            actor,
        })
    }
}

#[get("/back")]
async fn back() -> impl Responder {
    HttpResponse::Ok().body("Hello from Krusty!")
}

#[post("/back/login")]
async fn login() ->impl Responder {
    Redirect::to("/")
}