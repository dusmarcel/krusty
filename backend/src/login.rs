use std::sync::Mutex;

use actix_web::{post, web, Responder};
use serde::Deserialize;

use crate::{Backend, user::User};

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[post("/back/login")]
async fn login(backend: web::Data<Mutex<Backend>>, form: web::Form<FormData>) -> impl Responder {
    let my_backend = backend.lock().unwrap();
    let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE name = $1"
        )
        .bind(&form.username)
        .fetch_optional(&my_backend.pool)
        .await;

    match result {
        Ok(res) => {
            match res {
                Some(res) => {
                    println!("Found user: {:?}", res);
                    web::Redirect::to("/").see_other()
                },
                None => {
                    eprintln!("Login failed! User not foun.");
                    web::Redirect::to("/login").see_other()
                }
            }
        }                  
        Err(e) => {
            eprintln!("Login failed: {}", e);
            web::Redirect::to("/login").see_other()
        }   
    }
}