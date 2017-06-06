extern crate regex;
extern crate dictionary;

use std::env::args;
use std::fs;
use std::fmt::Debug;
use std::path::Path;
use std::io::Read;
use regex::Regex;
use dictionary::Dictionary;

fn learn_from_file<P: AsRef<Path> + Debug>(path: P, dict: &mut Dictionary) {
    println!("{:?}...", &path);

    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let rx_line =
        Regex::new(r"\(\d\d\d\d-\d\d-\d\d \d\d:\d\d:\d\d\)\s*<(?P<nick>[^>]+)> (?P<message>.*)")
            .unwrap();

    for line in contents.lines() {
        if let Some(caps) = rx_line.captures(line) {
            let nick = caps.name("nick").unwrap();
            let msg = caps.name("message").unwrap();

            if !nick.to_lowercase().contains("lucidbot") && !msg.starts_with('!') {
                dict.learn_from_line(msg);
            }
        }
    }
}

fn learn_from_dir<P: AsRef<Path>>(path: P, dict: &mut Dictionary) {
    let dir_content = fs::read_dir(path).unwrap();

    for entry in dir_content {
        if let Ok(entry) = entry {
            let filetype = entry.file_type().unwrap();
            if filetype.is_dir() {
                learn_from_dir(entry.path(), dict);
            } else if filetype.is_file() {
                learn_from_file(entry.path(), dict);
            }
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Required argument missing: base log directory");
        return;
    }
    let base_dir = &args[1];
    let mut dict = Dictionary::new();

    learn_from_dir(base_dir, &mut dict);

    let _ = dict.save("dictionary.dat");
}
