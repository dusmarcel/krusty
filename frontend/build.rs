use std::{env, fs};

fn main () {
    let backend_protocol=env::var("BACKEND_PROTOCOL").unwrap_or_else(|_| "http".to_string());
    let backend_host=env::var("BACKEND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let backend_port=env::var("BACKEND_PORT").unwrap_or_else(|_| "8161".to_string());

    let output=format!(r#"pub const BACKEND_URL: &str = "{}://{}:{}/back";"#, backend_protocol, backend_host, backend_port);

    fs::write("./backend_config.rs", output).expect("Could'nt write backend_config.rs");
}

