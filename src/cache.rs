
use std::{env, error::Error, fs, io, path::PathBuf};
use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub last_vqd: String,
    pub last_vqd_time: u64
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            last_vqd: "".into(),
            last_vqd_time: 0
        }
    }
}

impl Cache {

    pub fn get_path<T: From<String>>() -> T {
        match env::var("HEY_CACHE_PATH") {
            Ok(v) => v,
            Err(_) =>
                match home_dir() {
                    Some(home) => home.join(".cache/hey").as_os_str().as_encoded_bytes().iter().map(|x| char::from(*x)).collect::<String>(),
                    None => panic!("Cannot detect your home directory!")
                }
        }.into()
    }

    pub fn get_file_name<T: From<String>>() -> T {
        match env::var("HEY_CACHE_FILENAME") {
            Ok(v) => v,
            Err(_) => "cache.json".into()
        }.into()
    }

    fn ensure_dir_exists() -> io::Result<()> {
        let path: PathBuf = Self::get_path();
        if ! path.is_dir() { fs::create_dir_all(path)? }
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path: PathBuf = Self::get_path();
        Self::ensure_dir_exists()?;

        let file_path = path.join(Self::get_file_name::<PathBuf>());
        if ! file_path.is_file() {
            let def = Self::default();
            def.save()?;
            Ok(def)
        } else {
            let file = fs::read_to_string(file_path)?;
            Ok(serde_json::from_str(&file)?)
        }
    }

    pub fn save(self: &Self) -> Result<(), Box<dyn Error>> {
        let path: PathBuf = Self::get_path();
        Self::ensure_dir_exists()?;

        let file_path = path.join(Self::get_file_name::<PathBuf>());
        fs::write(file_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn set_last_vqd<T: Into<String>>(self: &mut Self, vqd: T) -> Result<(), Box<dyn Error>> {
        self.last_vqd = vqd.into();
        self.last_vqd_time = chrono::Local::now().timestamp_millis() as u64;
        self.save()?;
        Ok(())
    }

    pub fn get_last_vqd<'a, T: From<&'a String>>(self: &'a Self) -> Option<T> {
        None
        /*if self.last_vqd_time - (chrono::Local::now().timestamp_millis() as u64) < 60000 {
            Some((&self.last_vqd).into())
        } else {
            None
        } */
    }
}
