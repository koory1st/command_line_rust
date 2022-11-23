use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello_new").unwrap();
    cmd.assert().stdout("Hello, world!\n");
}