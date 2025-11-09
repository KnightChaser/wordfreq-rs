use std::process::Command;

#[test]
fn runs_on_stdin() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    use std::io::Write;
    let input = b"Rust rust RUST!";
    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let out = child.wait_with_output().unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert!(s.contains("rust"));
}
