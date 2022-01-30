use super::base::Command;

pub const COMMAND: Command = Command {
    name: "false",
    run: |_args: &[String]| -> i32 { 1 },
};
