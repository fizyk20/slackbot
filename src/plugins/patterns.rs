use plugin::Plugin;
use ::{BotEvent, ResumeEventHandling};

pub struct Patterns;

impl Plugin for Patterns {
    fn plugin_priority(&self, _: &str, _: &str, _: &str) -> i16 {
        -1
    }

    fn handle_message(&mut self, _: &str, _: &str, _: &str) -> BotEvent {
        BotEvent::Log("test".to_string(), ResumeEventHandling::Resume)
    }
}
