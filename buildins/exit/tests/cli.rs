use assert_cmd::Command;

#[test]
fn exit_with_status() {
    let mut cmd = Command::cargo_bin("exit").unwrap();
    let output = cmd.arg("2").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
}
