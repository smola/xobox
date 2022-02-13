use std::ffi::OsString;
use std::io::prelude::*;
use std::os::unix::ffi::OsStrExt;

use super::base::Command;

fn run(args: &[OsString]) -> i32 {
    // TODO: --help
    let s: Vec<u8> = if args.is_empty() {
        b"y\n".to_vec()
    } else {
        let args = &args.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let mut v: Vec<u8> = bstr::join(b" ", args);
        v.push(b'\n');
        v
    };
    let mut out = std::io::stdout();
    loop {
        if out.write(&s[..]).is_err() {
            break;
        }
    }

    0
}

pub const COMMAND: Command = Command { name: "yes", run };
