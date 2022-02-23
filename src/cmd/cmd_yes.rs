use std::ffi::OsString;
use std::io::prelude::*;
use std::os::unix::ffi::OsStrExt;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::base::Command;

const BUF_SIZE: usize = 8 * 1024;

fn run(args: &[OsString]) -> i32 {
    let line: Vec<u8> = if args.is_empty() {
        b"y\n".to_vec()
    } else {
        let args = &args.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let mut v: Vec<u8> = bstr::join(b" ", args);
        v.push(b'\n');
        v
    };

    ignore_sigpipe();

    let buf = if line.len() > BUF_SIZE {
        line
    } else {
        let buf_size = BUF_SIZE - BUF_SIZE % line.len();
        let mut buf: Vec<u8> = vec![0_u8; buf_size];
        let n_copies = BUF_SIZE / line.len();
        for i in 0..n_copies {
            buf[i * line.len()..(i + 1) * line.len()].copy_from_slice(&line);
        }
        buf
    };
    let mut out = std::io::stdout();
    loop {
        if out.write_all(&buf).is_err() {
            break;
        }
    }

    0
}

#[allow(unused_must_use)]
fn ignore_sigpipe() {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::signal::SIGPIPE, Arc::clone(&term));
}

pub const COMMAND: Command = Command { name: "yes", run };
