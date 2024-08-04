use std::{env, error::Error, fs, io, path::PathBuf};

use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    // outdated
    Claude12,
    GPT35,

    // current
    Claude,
    GPT3,
    Llama,
    Mixtral,
    GPT4OMini
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Self::Claude12 => panic!("Your config is outdated! Please delete your ~/.config/hey directory"),
            Self::GPT35 => panic!("Your config is outdated! Please delete your ~/.config/hey directory"),
            
            Self::Claude => String::from("claude-3-haiku-20240307"),
            Self::GPT3 => String::from("gpt-3.5-turbo-0125"),
            Self::Llama => String::from("meta-llama/Llama-3-70b-chat-hf"),
            Self::Mixtral => String::from("mistralai/Mixtral-8x7B-Instruct-v0.1"),
            Self::GPT4OMini => String::from("gpt-4o-mini")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub model: Model,
    pub tos: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: Model::Claude,
            tos: false
        }
    }
}

impl Config {
    pub fn get_path<T: From<String>>() -> T {
        match env::var("HEY_CONFIG_PATH") {
            Ok(v) => v,
            Err(_) => 
            match home_dir() {
                Some(home) => home.join(".config/hey").as_os_str().as_encoded_bytes().iter().map(|x| char::from(*x)).collect::<String>(),
                None => panic!("Cannot detect your home directory!")
            }
        }.into()
    }

    pub fn get_file_name<T: From<String>>() -> T {
        match env::var("HEY_CONFIG_FILENAME") {
            Ok(v) => v,
            Err(_) => "conf.toml".into()
        }.into()
    }

    fn ensure_dir_exists() -> io::Result<()> {
        let path: PathBuf = Self::get_path();
        if ! path.is_dir() { fs::create_dir_all(path)? }
        Ok(())
    }

    pub fn save(self: &Self) -> Result<(), Box<dyn Error>> {
        let path = Self::get_path::<PathBuf>();
        Self::ensure_dir_exists()?;

        let file_path = path.join(Self::get_file_name::<String>());
        fs::write(file_path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::get_path::<PathBuf>();

        let file_path = path.join(Self::get_file_name::<String>());
        if ! file_path.is_file() {
            Ok(Self::default())
        } else {
            let conf: Config = toml::from_str(&fs::read_to_string(file_path)?)?;
            conf.model.to_string(); // so that it would panic if the config is outdated
            
            Ok(conf)
        }
    }
}
