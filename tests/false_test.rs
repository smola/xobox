mod common;
use duct::cmd;

#[test]
fn test_false() {
    let res = cmd!(common::get_bin(), "false").unchecked().run().unwrap();
    assert_eq!(Some(1), res.status.code());
}
