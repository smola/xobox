mod common;
use duct::cmd;
use pretty_assertions::assert_eq;

#[test]
fn test_yes() {
    let res = cmd!(common::get_bin(), "yes")
        .pipe(cmd!("head", "-n", "5"))
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!("y\ny\ny\ny\ny\n", String::from_utf8(res.stdout).unwrap());
}

#[test]
fn test_yes_with_arg() {
    let res = cmd!(common::get_bin(), "yes", "yes")
        .pipe(cmd!("head", "-n", "5"))
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(
        "yes\nyes\nyes\nyes\nyes\n",
        String::from_utf8(res.stdout).unwrap()
    );
}
