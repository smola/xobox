use super::base::Command;

pub const COMMAND: Command = Command {
    name: "true",
    run: |_args: &[String]| -> i32 { 0 },
};
