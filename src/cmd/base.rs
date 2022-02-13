use std::ffi::OsString;

pub struct Command {
    pub name: &'static str,
    pub run: fn(&[OsString]) -> i32,
}
