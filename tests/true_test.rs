mod common;
use duct::cmd;
use pretty_assertions::assert_eq;

#[test]
fn test_true() {
    let res = cmd!(common::get_bin(), "true").run().unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!("", String::from_utf8(res.stdout).unwrap());
}
