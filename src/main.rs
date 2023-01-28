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

    let (_new_file_content, new_version) = match version.as_str().trim() {
        "patch" => update_version_by_label(file_content, VersionLabel::Patch),
        "minor" => update_version_by_label(file_content, VersionLabel::Minor),
        "major" => update_version_by_label(file_content, VersionLabel::Major),
        _ => update_version(file_content, String::from(version.trim())),
    };
    git_add();
    git_commit(&new_version);
    git_tag(&new_version);
}

fn _save_new_version(_new_file_content: String) {}

fn git_add() {
    let _ = Command::new("echo").arg("git add .").spawn();
}

fn git_commit(version: &str) {
    let _ = Command::new("echo")
        .arg(format!("git commit -m 'v{version}'"))
        .spawn();
}
fn git_tag(version: &str) {
    let _ = Command::new("echo")
        .arg(format!("git tag -a v{version} -m 'v{version}'"))
        .spawn();
}
