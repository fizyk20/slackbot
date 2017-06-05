use plugin::Plugin;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read};
use {BotEvent, ResumeEventHandling, MessageData};

pub struct Patterns {
    patterns: Vec<(Regex, String)>,
}

impl Patterns {
    pub fn new() -> io::Result<Patterns> {
        let mut file = try!(File::open("patterns.ini"));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));

        let mut patterns = Vec::new();
        let mut lines = contents.lines().peekable();
        while lines.peek().is_some() {
            let pattern = lines.next().unwrap();
            let response = lines
                .next()
                .expect("A pattern without a response detected!");
            patterns.push((Regex::new(pattern).unwrap(), response.to_string()));
        }

        Ok(Patterns { patterns: patterns })
    }
}

impl Plugin for Patterns {
    fn plugin_priority(&self, _: &str, _: &str, _: &str) -> i16 {
        -1
    }

    fn handle_message(&mut self, data: MessageData) -> BotEvent {
        for &(ref regex, ref response) in (&self.patterns).into_iter() {
            if regex.is_match(&data.msg.to_lowercase()) {
                return BotEvent::Send(response.clone(), ResumeEventHandling::Resume);
            }
        }
        BotEvent::None(ResumeEventHandling::Resume)
    }

    fn handle_command(&mut self, _: &str, _: &str, _: Vec<String>) -> BotEvent {
        BotEvent::None(ResumeEventHandling::Resume)
    }
}
