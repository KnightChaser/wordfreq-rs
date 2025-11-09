use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn runs_on_stdin() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"foo bar foo baz foo qux\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    let s = String::from_utf8(output.stdout).unwrap();

    assert!(s.contains("foo"));
    assert!(s.contains("3"));
}
