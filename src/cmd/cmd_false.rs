use super::base::Command;
use std::ffi::OsString;

pub const COMMAND: Command = Command {
    name: "false",
    run: |_args: &[OsString]| -> i32 { 1 },
};
