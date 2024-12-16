use std::{env, fs};

fn main () {
    let backend_if=env::var("BACKEND_IF").unwrap_or_else(|_| "127.0.0.1".to_string());

    let bc_output=format!(r#"pub const BACKEND_IF: &str = "{}";"#, backend_if);
    fs::write("./backend_config.rs", bc_output).expect("Could'nt write backend_config.rs");

    let postgres_host=env::var("POSTGRES_HOST").unwrap_or_else(|_| "db".to_string());
    let postgres_database=env::var("POSTGRES_DATABASE").unwrap_or_else(|_| "postgres".to_string());
    let postgres_user=env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let postgres_password=env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let postgres_url=format!("postgres://{}:{}@{}/{}",
        postgres_user,
        postgres_password,
        postgres_host,
        postgres_database
    );

    let postgres_output=format!(r#"pub const POSTGRES_URL: &str = "{}";"#, postgres_url);
    fs::write("./postgres_config.rs", postgres_output).expect("Could'nt write postgres_config.rs");

    // let valkey_host=env::var("VALKEY_HOST").unwrap_or_else(|_| "valkey".to_string());
    // let valkey_port=env::var("VALKEY_PORT").unwrap_or_else(|_| "6379".to_string());
    // let valkey_password=env::var("VALKEY_PASSWORD").unwrap_or_else(|_| "valkey".to_string());
    // let valkey_url=format!("redis://{}:{}?auth={}",
    //     valkey_host,
    //     valkey_port,
    //     valkey_password
    // );

    let valkey_host=env::var("VALKEY_HOST").unwrap_or_else(|_| "valkey".to_string());
    let valkey_port=env::var("VALKEY_PORT").unwrap_or_else(|_| "6379".to_string());
    let valkey_url=format!("rediss://{}:{}",
        valkey_host,
        valkey_port
    );

    let valkey_output=format!(r#"pub const VALKEY_URL: &str = "{}";"#, valkey_url);
    fs::write("./valkey_config.rs", valkey_output).expect("Could'nt write valkey_config.rs");    
}

