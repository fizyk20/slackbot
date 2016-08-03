use std::fs;
use std::path::Path;
use std::io::Read;

pub struct Settings {
    pub token: String
}

impl Settings {

    pub fn from_file<P: AsRef<Path>>(path: P) -> Settings {
        let mut file = fs::File::open(path).ok().unwrap();
        let mut settings = String::new();
        file.read_to_string(&mut settings).ok().expect("Couldn't read from file");

        // temporary
        Settings {
            token: settings.lines().next().unwrap().to_string()
        }
    }
}

lazy_static! {
    pub static ref SETTINGS : Settings = Settings::from_file("settings.ini");
}
