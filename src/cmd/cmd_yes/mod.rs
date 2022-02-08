use std::io::prelude::*;

use super::base::Command;

fn run(args: &[String]) -> i32 {
    // TODO: --help
    let mut s = String::new();
    if args.is_empty() {
        s.push_str("y\n");
    } else {
        s.push_str(args.join(" ").as_str());
        s.push('\n');
    }

    let b = s.as_bytes();
    let mut out = std::io::stdout();
    loop {
        if out.write(b).is_err() {
            break;
        }
    }

    0
}

pub const COMMAND: Command = Command {
    name: "yes",
    run,
};
