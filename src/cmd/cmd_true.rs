use super::base::Command;
use std::ffi::OsString;

pub const COMMAND: Command = Command {
    name: "true",
    run: |_args: &[OsString]| -> i32 { 0 },
};
