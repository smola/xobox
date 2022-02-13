mod common;
use duct::cmd;
use std::ffi::OsStr;

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
    assert_eq!(Vec::from("base\n"), res.stdout);
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
    assert_eq!(Vec::from("base\n"), res.stdout);
}
