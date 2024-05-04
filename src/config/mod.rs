mod config;

use crate::error::TodoError;
use std::{fs::File, io::{Read, Write}, sync::Mutex};

pub static CONFIG: Mutex<Option<config::Config>> = Mutex::new(None);
pub static SECRET: Mutex<Option<config::Secrets>> = Mutex::new(None);

pub static DEFAULT_CONFIG: &str = r#"
system: "clickup"
"#;

fn load_config() -> Result<(), TodoError> {
    println!("Loading config");
    let path = format!("{}/.todo/config.yaml", std::env::var("HOME").unwrap());

    let file_content: String = match File::open(path.clone()) {
        Ok(file) => {
            file.bytes()
                .map(|byte| byte.unwrap() as char)
                .collect::<String>()
        },
        Err(_) => {
            DEFAULT_CONFIG.to_string()
        }
    };

    let config = serde_yaml::from_str(&file_content)?;
    CONFIG.lock().unwrap().replace(config);

    Ok(())
}

fn load_secrets() -> Result<(), TodoError> {
    println!("Loading secrets");
    let path = format!("{}/.todo/credentials.yaml", std::env::var("HOME").unwrap());

    let file = match File::open(path.clone()) {
        Ok(file) => file,
        Err(_) => {
            std::path::Path::new("~/.todo");
            File::create(path)?
        }
    };

    let secrets = serde_yaml::from_reader(file)?;
    SECRET.lock().unwrap().replace(secrets);

    Ok(())
}

pub fn init() -> Result<(), TodoError> {
    load_config()?;
    load_secrets()?;

    Ok(())
}

pub fn get_system() -> String {
    CONFIG.lock().unwrap().as_ref().unwrap().system.clone()
}

pub fn get_api_key() -> String {
    SECRET.lock().unwrap().as_ref().unwrap().api_key.clone()
}
