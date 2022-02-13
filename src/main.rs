mod cmd;
mod help;

use std::ffi::OsString;

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();
    if args.len() == 1 {
        std::process::exit(help::run());
    }

    if args[1] == "--help" || args[1] == "-h" {
        std::process::exit(help::run());
    }

    // TODO: file on non-valid UTF-8 program
    let program = args[1].to_string_lossy();
    let args = &args[2..];

    for c in cmd::COMMANDS {
        if c.name == program {
            let code = (c.run)(args);
            std::process::exit(code);
        }
    }

    println!("Invalid program: {}", program);
    std::process::exit(1);
}
