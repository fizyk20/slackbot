use std::fs;
use std::path::Path;
use std::io::Read;
use std::borrow::Borrow;
use std::hash::Hash;
use std::collections::HashMap;
use regex::Regex;

pub struct Settings {
    pub token: String,
    pub command_char: String,
    other: HashMap<String, String>
}

impl Settings {

    pub fn from_file<P: AsRef<Path>>(path: P) -> Settings {
        let mut file = fs::File::open(path).ok().unwrap();
        let mut settings = String::new();
        file.read_to_string(&mut settings).ok().expect("Couldn't read from file");

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
            token: if let Some(token) = values.get("token") { token.to_string() } else { String::new() },
            command_char: if let Some(cmd) = values.get("command_char") { cmd.to_string() } else { "!".to_string() },
            other: values
        }
    }

    pub fn get_other<Q: ?Sized>(&self, key: &Q) -> Option<&String> 
        where String: Borrow<Q>, Q: Hash + Eq {
        self.other.get(key)
    }

}

lazy_static! {
    pub static ref SETTINGS : Settings = Settings::from_file("settings.ini");
}
