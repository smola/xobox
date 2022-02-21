use std::ffi::OsString;
use std::io::prelude::*;
use std::os::unix::ffi::OsStrExt;

use super::base::Command;

fn run(args: &[OsString]) -> i32 {
    // TODO: --help
    // TODO: [SUFFIX]
    if let Some(file) = args.get(0) {
        let mut out = std::io::stdout();
        let s = dirname(file.as_bytes());
        if out.write(s).is_err() {
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
    name: "dirname",
    run,
};

fn dirname(file: &[u8]) -> &[u8] {
    if file.is_empty() {
        return b".";
    }
    match file.iter().rposition(|c: &u8| c != &b'/') {
        None => b"/",
        Some(last_non_slash) => {
            let file = &file[0..=last_non_slash];
            match file.iter().rposition(|c: &u8| c == &b'/') {
                None => b".",
                Some(last_slash) => {
                    let file = &file[..last_slash];
                    match file.iter().rposition(|c: &u8| c == &b'/') {
                        None if file.is_empty() => b"/",
                        None => file,
                        Some(prev_slash) => &file[prev_slash + 1..],
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(dirname(b""), b".");
    }

    #[test]
    fn test_slash() {
        assert_eq!(dirname(b"/"), b"/");
    }

    #[test]
    fn test_slashes() {
        assert_eq!(dirname(b"//"), b"/");
        assert_eq!(dirname(b"///"), b"/");
        assert_eq!(dirname(b"////"), b"/");
    }

    #[test]
    fn test_simple() {
        assert_eq!(dirname(b"basename"), b".");
        assert_eq!(dirname(b"basename/"), b".");
        assert_eq!(dirname(b"basename//"), b".");
        assert_eq!(dirname(b"/basename"), b"/");
        assert_eq!(dirname(b"other/basename"), b"other");
        assert_eq!(dirname(b"/other/basename"), b"other");
    }
}
