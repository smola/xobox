use super::base::Command;
use std::ffi::OsString;

fn run(_args: &[OsString]) -> i32 {
    1
}

pub const COMMAND: Command = Command { name: "false", run };
