use super::server_manager::Manager;
use std::error::Error;

/// Represents some piece of code that wants to do something with
/// each line of output from the server, such as send a message, log
/// player activity, or otherwise use information from the server's
/// output.
pub trait LineUser {
    /// Accepts one line of information through the line parameter,
    fn handle_line(&mut self, line: &str);
}

/// Represents an action for doing something to a running server
/// instance, such as making a backup or sending a command.
pub trait ScheduledEvent {
    /// Return the cron schedule for the task as an 
    /// `Option::Some<String>` if it shoud be regularly scheduled, or
    /// `Option::None` if not.
    fn get_schedule(&self) -> Option<String> {
        None
    }

    /// Take an instance of `Manager` and do something with it.
    fn execute(&mut self, server: &mut Manager) -> Result<(), Box<dyn Error>>;
}