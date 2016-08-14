use plugin::Plugin;
use ::{BotEvent, ResumeEventHandling, MessageData};
use dictionary::Dictionary;
use settings::SETTINGS;
use rand::{self, Rng};
use std::str::FromStr;

pub struct RandomChat {
    dict: Dictionary,
    enabled: bool,
    probability: u8
}

impl RandomChat {
    pub fn new() -> RandomChat {
        let dict = Dictionary::load("dictionary.dat").unwrap();
        let settings = SETTINGS.lock().unwrap();
        RandomChat {
            dict: dict,
            enabled: settings.get_other("randomchat_enabled").unwrap() == "true",
            probability: FromStr::from_str(settings.get_other("randomchat_probability").unwrap()).unwrap()
        }
    }
}

impl Plugin for RandomChat {
    fn plugin_priority(&self, _: &str, _: &str, _: &str) -> i16 {
        10
    }

    fn handle_message(&mut self, data: MessageData) -> BotEvent {
        if !self.enabled {
            return BotEvent::None(ResumeEventHandling::Resume);
        }
        if rand::thread_rng().gen_range(0, 100) < self.probability {
            let response = self.dict.generate_sentence();
            BotEvent::Send(response, ResumeEventHandling::Resume)
        }
        else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }

    fn handle_command(&mut self, _: &str, _: &str, params: Vec<String>) -> BotEvent {
        if params[0] == "gadaj" {
            let response = self.dict.generate_sentence();
            BotEvent::Send(response, ResumeEventHandling::Stop)
        }
        else if params[0] == "random" {
            if params.len() < 2 {
                return BotEvent::Send(String::from("Not enough parameters"), ResumeEventHandling::Stop);
            }
            if params[1] == "enable" {
                self.enabled = true;
                SETTINGS.lock().unwrap().set_other("randomchat_enabled".to_string(), "true".to_string());
                return BotEvent::Send(String::from("RandomChat enabled."), ResumeEventHandling::Stop);
            }
            else if params[1] == "disable" {
                self.enabled = false;
                SETTINGS.lock().unwrap().set_other("randomchat_enabled".to_string(), "false".to_string());
                return BotEvent::Send(String::from("RandomChat disabled."), ResumeEventHandling::Stop);
            }
            else {
                return BotEvent::Send(format!("Unknown parameter value: {}", params[1]).to_string(), ResumeEventHandling::Stop);
            }
        }
        else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }
}
