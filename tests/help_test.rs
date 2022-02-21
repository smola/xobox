mod common;
use duct::cmd;
use pretty_assertions::assert_eq;

#[test]
fn test_help() {
    let res = cmd!(common::get_bin())
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(
        "xobox\n\nFunctions:\n\tbasename\n\tdirname\n\tfalse\n\ttrue\n\tyes\n",
        String::from_utf8(res.stdout).unwrap()
    );
}

#[test]
fn test_help_flag_help() {
    let res = cmd!(common::get_bin(), "--help")
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(
        "xobox\n\nFunctions:\n\tbasename\n\tdirname\n\tfalse\n\ttrue\n\tyes\n",
        String::from_utf8(res.stdout).unwrap()
    );
}

#[test]
fn test_help_flag_h() {
    let res = cmd!(common::get_bin(), "-h")
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(
        "xobox\n\nFunctions:\n\tbasename\n\tdirname\n\tfalse\n\ttrue\n\tyes\n",
        String::from_utf8(res.stdout).unwrap()
    );
}

#[test]
fn test_help_invalid_program() {
    let res = cmd!(common::get_bin(), "bad")
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(1), res.status.code());
    assert_eq!(
        "Invalid program: bad\n",
        String::from_utf8(res.stdout).unwrap()
    );
}
