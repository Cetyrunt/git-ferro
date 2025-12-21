mod config_deserializer;
mod profile;
use config_deserializer::parser;
use config_deserializer::scanner_logic::scanner as toml;
use std::{env, fs};

fn main() -> Result<(), String> {
    let arg1: String = env::args().nth(1).unwrap_or_else(|| "jannis".to_string());
    let toml: String = fs::read_to_string("/home/rafael/Documents/Coding/Rust/git-ferro/test.toml")
        .expect("Should have been able to read the file");
    let output = toml::Scanner::deserialize(&toml);

    let table: std::collections::HashMap<String, parser::Value> = parser::Parser::run(output)?;

    let cred = profile::Credential::get_config(&table, &arg1)?;

    let default_username = extract_string(&table, "default_username")?;

    println!("default username: {:?}", default_username);

    println!("GitHub host: {:?}", cred.host);
    println!("GitHub protocol: {:?}", cred.protocol);
    println!("GitHub username: {:?}", cred.username);
    println!("GitHub token: {:?}", cred.token);

    Ok(())
}

fn extract_string(table: &parser::Table, key: &str) -> Result<String, String> {
    match table.get(key) {
        Some(parser::Value::String(s)) => Ok(s.to_string()),

        Some(_) => Err(format!("Expected string for '{}'", key)),

        None => Err(format!("Missing key '{}'", key)),
    }
}
