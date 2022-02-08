mod common;
use duct::cmd;

#[test]
fn test_yes() {
    let res = cmd!(common::get_bin(), "yes")
        .pipe(cmd!("head", "-n", "5"))
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    assert_eq!(Some(0), res.status.code());
    assert_eq!(Vec::from("y\ny\ny\ny\ny\n"), res.stdout);
}
