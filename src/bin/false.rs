use assert_cmd::Command;

pub fn main() {
    std::process::abort();
}

#[test]
fn false_failure() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
