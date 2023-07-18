use assert_cmd::Command;

#[test]
fn test1() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.arg("1").assert().success().stdout("3.1416");
}

#[test]
fn test2() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.arg("2").assert().success().stdout("12.5664");
}
