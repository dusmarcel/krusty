use async_std::stream::StreamExt;

use actix_web::{post, web, Responder};
use serde::Deserialize;

use crate::{Backend, user::User};

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[post("/back/login")]
async fn login(backend: web::Data<Backend>, form: web::Form<FormData>) ->impl Responder {
    let mut stream = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE name = $1"
        )
        .bind(&form.username)
        .fetch(&backend.pool);

    match stream.next().await {
        Some(res) => {
            match res {
                Ok(res) => {
                    println!("Found user: {:?}", res);
                    web::Redirect::to("/").see_other()
                },
                Err(e) => {
                    eprintln!("Login failed: {}", e);
                    web::Redirect::to("/login").see_other()
                }
            }
        }                  
        None => {
            eprintln!("Login failed!");
            web::Redirect::to("/login").see_other()
        }   
    }
}