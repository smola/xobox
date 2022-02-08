mod cmd;
mod help;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        std::process::exit(help::run());
    }

    if args[1] == "--help" || args[1] == "-h" {
        std::process::exit(help::run());
    }

    let program = &args[1];
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
