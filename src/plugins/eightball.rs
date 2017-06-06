use {BotEvent, MessageData, ResumeEventHandling};
use plugin::Plugin;
use rand::{sample, thread_rng};
use std::fs::File;
use std::io::{self, Read};

pub struct Eightball {
    responses: Vec<String>,
}

impl Eightball {
    pub fn new() -> io::Result<Eightball> {
        let mut file = File::open("eightball.ini")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Eightball { responses: contents.lines().map(|s| s.to_owned()).collect() })
    }
}

impl Plugin for Eightball {
    fn plugin_priority(&self, _: &str, _: &str, _: &str) -> i16 {
        -1
    }

    fn handle_message(&mut self, _: MessageData) -> BotEvent {
        BotEvent::None(ResumeEventHandling::Resume)
    }

    fn handle_command(&mut self, user: &str, _: &str, params: Vec<String>) -> BotEvent {
        if params[0] == "eightball" {
            let mut rng = thread_rng();
            let sample = sample(&mut rng, &self.responses, 1);
            let result = sample[0].replace("%s", user);
            BotEvent::Send(result, ResumeEventHandling::Stop)
        } else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }
}
