use assert_cmd::Command;

#[test]
fn true_success() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
