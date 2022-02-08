pub mod base;
mod cmd_false;
mod cmd_true;
mod cmd_yes;

pub const COMMANDS: &[base::Command] = &[cmd_false::COMMAND, cmd_true::COMMAND, cmd_yes::COMMAND];
