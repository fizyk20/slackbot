use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::borrow::Borrow;
use std::hash::Hash;
use std::collections::HashMap;
use regex::Regex;

pub struct Settings {
    path: PathBuf,
    pub token: String,
    pub command_char: String,
    other: HashMap<String, String>,
}

impl Settings {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Settings {
        let path_buf = path.as_ref().to_path_buf();
        let mut file = fs::File::open(path).ok().unwrap();
        let mut settings = String::new();
        file.read_to_string(&mut settings)
            .ok()
            .expect("Couldn't read from file");

        let mut values = HashMap::new();
        let re = Regex::new(r#""([^"]+)"\s*:\s*"([^"]+)""#).unwrap();

        for line in settings.lines() {
            if let Some(caps) = re.captures(line) {
                let key = caps.at(1).unwrap();
                let value = caps.at(2).unwrap();
                values.insert(key.to_string(), value.to_string());
            }
        }

        Settings {
            path: path_buf,
            token: if let Some(token) = values.get("token") {
                token.to_string()
            } else {
                String::new()
            },
            command_char: if let Some(cmd) = values.get("command_char") {
                cmd.to_string()
            } else {
                "!".to_string()
            },
            other: values,
        }
    }

    pub fn save(&self) {
        let mut file = fs::File::create(&self.path).unwrap();
        for (key, value) in self.other.iter() {
            let line = format!("\"{}\" : \"{}\"\n", key, value);
            let _ = file.write(line.as_bytes());
        }
    }

    pub fn get_other<Q: ?Sized>(&self, key: &Q) -> Option<&String>
        where String: Borrow<Q>,
              Q: Hash + Eq
    {
        self.other.get(key)
    }

    pub fn set_other(&mut self, key: String, value: String) {
        self.other.insert(key, value);
        self.save();
    }
}

lazy_static! {
    pub static ref SETTINGS : ::std::sync::Mutex<Settings> = ::std::sync::Mutex::new(Settings::from_file("settings.ini"));
}
