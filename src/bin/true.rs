use assert_cmd::Command;

pub fn main() {
    std::process::exit(0);
}

#[test]
fn true_success() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
