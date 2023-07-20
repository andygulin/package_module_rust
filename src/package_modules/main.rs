use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let projects = &args[1..].to_vec();
    Command::new("mvn")
        .arg("clean")
        .arg("package")
        .arg("-pl")
        .args(projects)
        .arg("-am").exec();
}