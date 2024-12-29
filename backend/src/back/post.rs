use std::sync::Mutex;

use actix_web::{post, web, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    back::{
        Backend,
        User,
    },
    ap::activity::Activity
};

#[derive(Deserialize)]
struct FormData {
    in_reply_to: Option<String>,
    post: String
}

#[post("/post")]
async fn post(backend: web::Data<Mutex<Backend>>, session: Session, form: web::Json<FormData>) -> impl Responder {
    if form.post.is_empty() {
        return HttpResponse::BadRequest().body("Post cannot be empty!");
    }

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
                        if let Some(user) = res {
                            if let Some(host) = &my_backend.host {
                                let activity = Activity::new(host, &user, &form.in_reply_to, &form.post);
                                HttpResponse::Ok().json(activity.to_shared())
                            } else {
                                eprintln!("Host was not set!");
                                HttpResponse::InternalServerError().body("Internal Server error!")
                            }
                        } else {
                            eprintln!("id was found, but was not valid.");
                            HttpResponse::InternalServerError().body("Internal Server error!")
                        }
                    }
                    Err(e) => {
                        eprintln!("Error while executing query: {}", e);
                        HttpResponse::InternalServerError().body("Internal Server error!")
                    }
                }
            } else {
                eprintln!("id was found, but was not valid.");
                HttpResponse::InternalServerError().body("Internal Server error!")
            }
        } else {
            eprintln!("id was found, but was not valid.");
            HttpResponse::InternalServerError().body("Internal Server error!")
        }
    } else {
        HttpResponse::Unauthorized().body("You are not logged in!")
    }
}