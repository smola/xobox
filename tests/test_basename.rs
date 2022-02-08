mod common;
use duct::cmd;

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
