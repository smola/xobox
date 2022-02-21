use super::base::Command;
use std::ffi::OsString;

fn run(_args: &[OsString]) -> i32 {
    0
}

pub const COMMAND: Command = Command { name: "true", run };
