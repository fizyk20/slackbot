use ::BotEvent;

pub trait Plugin {
    fn plugin_priority(&self, user: &str, channel: &str, msg: &str) -> i16;
    fn handle_command(&mut self, user: &str, channel: &str, msg: &str) -> BotEvent;
    fn handle_message(&mut self, user: &str, channel: &str, msg: &str) -> BotEvent;
}
