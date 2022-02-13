mod common;
use duct::cmd;
use pretty_assertions::assert_eq;
use std::ffi::OsStr;
use std::string::String;

#[cfg(not(windows))]
use std::os::unix::ffi::OsStrExt;

#[test]
fn test_basename() {
    let res = cmd!(common::get_bin(), "basename", "other/base")
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!("base\n", String::from_utf8(res.stdout).unwrap());
}

#[test]
fn test_basename_no_arg() {
    let res = cmd!(common::get_bin(), "basename")
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(1), res.status.code());
    // TODO: print help
    assert_eq!("", String::from_utf8(res.stdout).unwrap());
}

#[cfg(not(tarpaulin))]
#[test]
fn test_basename_no_stdout() {
    let res = cmd!(common::get_bin(), "basename", "other/base")
        .pipe(cmd!("true"))
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    // XXX: exit code of true, not basename
    assert_eq!(Some(0), res.status.code());
    assert_eq!("", String::from_utf8(res.stdout).unwrap());
}

#[cfg(not(windows))]
#[test]
fn test_basename_non_utf8() {
    let res = cmd!(
        common::get_bin(),
        OsStr::from_bytes(b"basename"),
        OsStr::from_bytes(b"\xFF/base")
    )
    .stdout_capture()
    .unchecked()
    .run()
    .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!("base\n", String::from_utf8(res.stdout).unwrap());
}
