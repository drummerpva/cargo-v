use cargo_v::{update_version, update_version_by_label, VersionLabel};
use std::{env, fs, process::Command};
fn main() {
    let mut args = env::args();
    args.next();
    dbg!(&args);
    let version = args.next().expect("You must pass the version");

    let file = fs::read_to_string("./Cargo.toml");
    let file_content = match file {
        Ok(data) => data,
        Err(err) => panic!("Can not load file: {err}"),
    };
    

    git_add();
    git_commit(&version);
}

fn git_add() {
    let _ = Command::new("echo").arg("git add .").spawn();
}

fn git_commit(version: &str) {
    let _ = Command::new("echo")
        .arg(format!("git commit -m '{version}'"))
        .spawn();
}
