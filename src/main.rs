use console::Term;
use std::process::Stdio;
use tokio::process::Command;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let term = Term::stderr();
    let mut child = Command::new("sleep")
        .arg("2")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let response = term.read_line().unwrap();
    child.wait().await.unwrap();
    println!("{response}");
}
