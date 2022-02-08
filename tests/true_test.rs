mod common;
use duct::cmd;

#[test]
fn test_true() {
    let res = cmd!(common::get_bin(), "true").run().unwrap();
    assert_eq!(Some(0), res.status.code());
}
