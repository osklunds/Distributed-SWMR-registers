
use std::process::Command;
use std::str;
use std::{thread, time};


fn main() {
    let mut child = Command::new("/bin/bash")
                .env("RED", "\\033[0;31m")
                .arg("-c")
                .arg("printf ${RED}")
                .arg("cargo run --manifest-path ../application/Cargo.toml --color always")
                .spawn()
                .expect("failed to execute process");


    thread::sleep(time::Duration::from_millis(5000));
    child.kill();
}
