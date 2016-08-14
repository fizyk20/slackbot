use plugin::Plugin;
use ::{BotEvent, ResumeEventHandling};
use dictionary::Dictionary;

pub struct RandomChat {
    dict: Dictionary
}

impl RandomChat {
    pub fn new() -> RandomChat {
        let dict = Dictionary::load("dictionary.dat").unwrap();
        RandomChat {
            dict: dict
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

    fn handle_command(&mut self, _: &str, _: &str, msg: &str) -> BotEvent {
        if msg.starts_with("gadaj") {
            let response = self.dict.generate_sentence();
            BotEvent::Send(response, ResumeEventHandling::Stop)
        }
        else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }
}
