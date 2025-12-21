use crate::config_deserializer::parser::Table;
use crate::config_deserializer::parser::Value;

#[derive(Debug, Default)]
pub struct Credential {
    pub host: String,
    pub protocol: String,
    pub username: String,
    pub token: String,
}

fn extract_string(table: &Table, key: &str) -> Result<String, String> {
    match table.get(key) {
        Some(Value::String(s)) => Ok(s.to_string()),
        Some(_) => Err(format!("Expected string for '{}'", key)),
        None => Err(format!("Missing key '{}'", key)),
    }
}

fn parse_credential(table: &Table, user_name: &str) -> Result<Credential, String> {
    match table.get("credential") {
        Some(Value::Array(arr)) => {
            for t in arr {
                let name = extract_string(t, "username")?;
                if name != user_name {
                    continue;
                }

                return Ok(Credential {
                    host: extract_string(t, "host")?,
                    protocol: extract_string(t, "protocol")?,
                    username: name,
                    token: extract_string(t, "token")?,
                });
            }

            Err(format!("No credential found for host '{}'", user_name))
        }
        Some(_) => Err("Expected array of tables for 'credential'".into()),
        None => Err("Missing 'credential' section".into()),
    }
}

impl Credential {
    pub fn get_config(table: &Table, user_name: &str) -> Result<Credential, String> {
        parse_credential(&table, user_name)
    }
}
