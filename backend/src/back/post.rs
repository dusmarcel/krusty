use std::sync::Mutex;

use actix_web::{post, web, HttpResponse, Responder};
use actix_session::Session;
use awc::Client;
use base64::prelude::*;
use chrono::Utc;
use openssl::{hash::{Hasher, MessageDigest}, pkey::PKey, rsa::Rsa, sign::Signer};
use serde::Deserialize;
use serde_json;
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
    in_reply_to: String,
    content: String
}

#[post("/post")]
async fn post(backend: web::Data<Mutex<Backend>>, session: Session, form: web::Json<FormData>) -> impl Responder {
    if form.content.is_empty() {
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
                                let activity = Activity::new(host, &user, &form.in_reply_to, &form.content);
                                println!("Activity: {:#?}", activity);

                                let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
                                hasher.update(serde_json::to_string(&activity.to_shared()).unwrap().as_bytes()).unwrap();
                                let digest_bytes = hasher.finish().unwrap();
                                let digest = format!("SHA-256={}", BASE64_STANDARD_NO_PAD.encode(digest_bytes));
                                let date = Utc::now().to_rfc2822();
                                let rsa = Rsa::private_key_from_pem(user.private_key.as_bytes()).unwrap();
                                let priv_key = PKey::from_rsa(rsa).unwrap();
                                let post_host = "mastodon.social"; // for testing purposes
                                let signed_string = format!(
                                    "(request-target): post /inbox\nhost: {}\ndate: {}\ndigest: {}\n",
                                    post_host,
                                    date,
                                    digest
                                );
                                println!("String to sign: {}", signed_string);
                                let mut signer = Signer::new(MessageDigest::sha256(), &priv_key).unwrap();
                                signer.update(signed_string.as_bytes()).unwrap();
                                let signature = signer.sign_to_vec().unwrap();
                                let signature = BASE64_STANDARD_NO_PAD.encode(signature);

                                let header = format!(
                                    "keyId=\"https://{}/user/{}\",headers=\"(request-target) host date digest\",signature=\"{}\"",
                                    host,
                                    user.preferred_username,
                                    signature
                                );
                                println!("Authorization header: {}", header);

                                let client = Client::default();
                                let response = client.post(format!("https://{}/inbox", post_host))
                                    .insert_header(("Host", post_host))
                                    .insert_header(("Date", date))
                                    .insert_header(("Digest", digest))
                                    .insert_header(("Authorization", header))
                                    .send_json(&activity.to_shared())
                                    .await;

                                if let Ok(response) = response {
                                    if response.status().is_success() {
                                        println!("Successfully posted activity!");
                                    } else {
                                        eprintln!("Failed to post activity: {}", response.status());
                                        return HttpResponse::InternalServerError().body("Internal Server error!");
                                    }
                                } else {
                                    eprintln!("Failed to post activity: {}", response.unwrap_err());
                                    return HttpResponse::InternalServerError().body("Internal Server error!");
                                }

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