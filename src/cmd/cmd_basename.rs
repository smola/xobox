use std::io::prelude::*;

use super::base::Command;

fn run(args: &[String]) -> i32 {
    // TODO: --help
    // TODO: [SUFFIX]
    if let Some(file) = args.get(0) {
        let mut out = std::io::stdout();
        let s = basename(file);
        if out.write(s.as_bytes()).is_err() {
            return 1;
        }
        if out.write("\n".as_bytes()).is_err() {
            return 1;
        }
        return 0;
    }
    1
}

pub const COMMAND: Command = Command {
    name: "basename",
    run,
};

fn basename(file: &str) -> String {
    if file.is_empty() {
        return String::from("");
    }
    match file.rfind(|c: char| c != '/') {
        None => String::from("/"),
        Some(last_non_slash) => {
            let file = &file[0..=last_non_slash];
            match file.rfind(|c: char| c == '/') {
                None => String::from(file),
                Some(last_slash) => String::from(&file[last_slash + 1..]),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(basename(&String::from("")), String::from(""));
    }

    #[test]
    fn test_slash() {
        assert_eq!(basename(&String::from("/")), String::from("/"));
    }

    #[test]
    fn test_slashes() {
        assert_eq!(basename(&String::from("//")), String::from("/"));
        assert_eq!(basename(&String::from("///")), String::from("/"));
        assert_eq!(basename(&String::from("////")), String::from("/"));
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            basename(&String::from("basename")),
            String::from("basename")
        );
        assert_eq!(
            basename(&String::from("basename/")),
            String::from("basename")
        );
        assert_eq!(
            basename(&String::from("basename//")),
            String::from("basename")
        );
        assert_eq!(
            basename(&String::from("/basename")),
            String::from("basename")
        );
        assert_eq!(
            basename(&String::from("other/basename")),
            String::from("basename")
        );
        assert_eq!(
            basename(&String::from("/other/basename")),
            String::from("basename")
        );
    }
}
