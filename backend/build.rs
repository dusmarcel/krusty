use std::{env, fs};

fn main () {
    let backend_if=env::var("BACKEND_IF").unwrap_or_else(|_| "127.0.0.1".to_string());

    let bc_output=format!(r#"pub const BACKEND_IF: &str = "{}";"#, backend_if);

    fs::write("./backend_config.rs", bc_output).expect("Could'nt write backend_config.rs");

    let postgres_host=env::var("POSTGRES_HOST").unwrap_or_else(|_| "db".to_string());
    let postgres_database=env::var("POSTGRES_DATABASE").unwrap_or_else(|_| "postgres".to_string());
    let postgres_user=env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let postgres_password=env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

    let db_output=format!(r#"pub const DATABASE_URL: &str = "postgres://{}:{}@{}/{}";"#,
        postgres_user,
        postgres_password,
        postgres_host,
        postgres_database
    );

    fs::write("./database_config.rs", db_output).expect("Could'nt write database_config.rs");
}

