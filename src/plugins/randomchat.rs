use {BotEvent, MessageData, ResumeEventHandling};
use chrono::Duration;
use dictionary::Dictionary;
use plugin::Plugin;
use rand::{self, Rng};
use settings::SETTINGS;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use timer::{Guard, Timer};

pub struct RandomChat {
    dict: Arc<Mutex<Dictionary>>,
    enabled: bool,
    probability: u8,
    autosave_timer: Option<Timer>,
    autosave_guard: Option<Guard>,
}

impl RandomChat {
    pub fn new() -> RandomChat {
        let dict = Dictionary::load("dictionary.dat").unwrap();
        let settings = SETTINGS.lock().unwrap();
        RandomChat {
            dict: Arc::new(Mutex::new(dict)),
            enabled: settings.get_other("randomchat_enabled").unwrap() == "true",
            probability: FromStr::from_str(settings.get_other("randomchat_probability").unwrap())
                .unwrap(),
            autosave_timer: None,
            autosave_guard: None,
        }
    }

    fn init_timer(&mut self) {
        if self.autosave_timer.is_none() {
            self.autosave_timer = Some(Timer::new());
            self.autosave_guard = {
                let dict = self.dict.clone();
                Some(self.autosave_timer
                         .as_ref()
                         .unwrap()
                         .schedule_repeating(Duration::minutes(10), move || {
                    let _ = dict.lock().unwrap().save("dictionary.dat");
                }))
            }
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
        self.init_timer();
        if data.self_name != data.user {
            self.dict.lock().unwrap().learn_from_line(data.msg);
        }
        if rand::thread_rng().gen_range(0, 100) < self.probability {
            let response = self.dict.lock().unwrap().generate_sentence();
            BotEvent::Send(response, ResumeEventHandling::Resume)
        } else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }

    fn handle_command(&mut self, _: &str, _: &str, params: Vec<String>) -> BotEvent {
        if params[0] == "gadaj" {
            let response = self.dict.lock().unwrap().generate_sentence();
            BotEvent::Send(response, ResumeEventHandling::Stop)
        } else if params[0] == "random" {
            if params.len() < 2 {
                return BotEvent::Send(String::from("Not enough parameters"),
                                      ResumeEventHandling::Stop);
            }
            if params[1] == "enable" {
                self.enabled = true;
                SETTINGS
                    .lock()
                    .unwrap()
                    .set_other("randomchat_enabled".to_string(), "true".to_string());
                BotEvent::Send(String::from("RandomChat enabled."),
                               ResumeEventHandling::Stop)
            } else if params[1] == "disable" {
                self.enabled = false;
                SETTINGS
                    .lock()
                    .unwrap()
                    .set_other("randomchat_enabled".to_string(), "false".to_string());
                BotEvent::Send(String::from("RandomChat disabled."),
                               ResumeEventHandling::Stop)
            } else {
                BotEvent::Send(format!("Unknown parameter value: {}", params[1]).to_string(),
                               ResumeEventHandling::Stop)
            }
        } else {
            BotEvent::None(ResumeEventHandling::Resume)
        }
    }
}
