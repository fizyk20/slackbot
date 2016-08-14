use plugin::Plugin;
use ::{BotEvent, ResumeEventHandling};
use dictionary::Dictionary;
use settings::SETTINGS;

pub struct RandomChat {
    dict: Dictionary,
    enabled: bool
}

impl RandomChat {
    pub fn new() -> RandomChat {
        let dict = Dictionary::load("dictionary.dat").unwrap();
        RandomChat {
            dict: dict,
            enabled: SETTINGS.get_other("randomchat_enabled").unwrap() == "true"
        }
    }
}

impl Plugin for RandomChat {
    fn plugin_priority(&self, _: &str, _: &str, _: &str) -> i16 {
        10
    }

    fn handle_message(&mut self, _: &str, _: &str, _: &str) -> BotEvent {
        BotEvent::None(ResumeEventHandling::Resume)
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
            }
            else if params[1] == "disable" {
                self.enabled = false;
            }
            else {
                return BotEvent::Send(format!("Unknown parameter value: {}", params[1]).to_string(), ResumeEventHandling::Stop);
            }
            BotEvent::None(ResumeEventHandling::Resume)
        }
        else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }
}
