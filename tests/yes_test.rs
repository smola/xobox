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

#[test]
fn test_yes_with_large_arg() {
    let arg = std::iter::repeat("X").take(9000).collect::<String>();
    let res = cmd!(common::get_bin(), "yes", arg.clone())
        .pipe(cmd!("head", "-n", "1"))
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(arg + "\n", String::from_utf8(res.stdout).unwrap());
}

#[test]
fn test_yes_with_multiple_arg() {
    let res = cmd!(common::get_bin(), "yes", "yes", "no")
        .pipe(cmd!("head", "-n", "5"))
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(
        "yes no\nyes no\nyes no\nyes no\nyes no\n",
        String::from_utf8(res.stdout).unwrap()
    );
}
