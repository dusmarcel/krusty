use std::sync::Mutex;

use actix_session::Session;
use actix_web::{http::StatusCode, post, web, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Deserialize;

use crate::{Backend, user::User};

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[post("/back/login")]
async fn login(backend: web::Data<Mutex<Backend>>, session: Session, form: web::Form<FormData>) -> impl Responder {
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
                Some(user) => {
                    match PasswordHash::new(&user.hash) {
                        Ok(hash) => {
                            match Argon2::default().verify_password(form.password.as_bytes(), &hash) {
                                Ok(_) => {
                                    session.renew();
                                    match session.insert("id", user.id.to_string()) {
                                        Ok(_) => {
                                            println!("Login succesful!");
                                            web::Redirect::to("/").see_other()
                                        }
                                        Err(e) => {
                                            eprintln!("Login failed: {}", e);
                                            web::Redirect::to("/login").using_status_code(StatusCode::INTERNAL_SERVER_ERROR)
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Login failed: {}", e);
                                    web::Redirect::to("/login").using_status_code(StatusCode::FORBIDDEN)
                                }
                            }
                            
                        }
                        Err(e) => {
                            eprintln!("Could'nt retrieve password hash form database: {}", e);
                            web::Redirect::to("/login").using_status_code(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                },
                None => {
                    eprintln!("Login failed! User not found.");
                    web::Redirect::to("/login").using_status_code(StatusCode::FORBIDDEN)
                }
            }
        }                  
        Err(e) => {
            eprintln!("Login failed: {}", e);
            web::Redirect::to("/login").using_status_code(StatusCode::INTERNAL_SERVER_ERROR)
        }   
    }
}