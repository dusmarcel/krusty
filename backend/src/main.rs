#[macro_use] extern crate rocket;

use openssl::rsa::Rsa;

#[get("/")]
fn index() -> String {
    let rsa = Rsa::generate(2048).unwrap();
    match rsa.private_key_to_pem() {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(e) => format!("Couldn't read private key. The error message was: {}", e),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
