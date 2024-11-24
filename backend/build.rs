use std::{env, fs};

fn main () {
    let backend_if=env::var("BACKEND_IF").unwrap_or_else(|_| "127.0.0.1".to_string());

    let output=format!(r#"pub const BACKEND_IF: &str = "{}";"#, backend_if);

    fs::write("./backend_config.rs", output).expect("Could'nt write backend_config.rs");
}

