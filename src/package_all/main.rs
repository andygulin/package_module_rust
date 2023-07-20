use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    Command::new("mvn")
        .arg("clean")
        .arg("package").exec();
}