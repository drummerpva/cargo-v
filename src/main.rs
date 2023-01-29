use cargo_v::{update_version, update_version_by_label, VersionLabel};
use std::{
    env,
    error::Error,
    fs,
    process::{self, Command},
};
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

    let (new_file_content, new_version) = match version.as_str().trim() {
        "patch" => update_version_by_label(file_content, VersionLabel::Patch),
        "minor" => update_version_by_label(file_content, VersionLabel::Minor),
        "major" => update_version_by_label(file_content, VersionLabel::Major),
        _ => update_version(file_content, String::from(version.trim())),
    };
    if save_new_version_in_cargo_toml(new_file_content).is_err() {
        println!("Erro on Save new content att Cargo.toml");
    }
    run_build();
    git_add();
    git_commit(&new_version);
    git_tag(&new_version);
    process::exit(0);
}
fn save_new_version_in_cargo_toml(new_file_content: String) -> Result<(), Box<dyn Error>> {
    fs::write("./Cargo.toml", new_file_content)?;
    Ok(())
}
fn run_build() {
    let _ = Command::new("cargo").args(["build", "--release"]).spawn();
}

fn git_add() {
    let _ = Command::new("git")
        .args(["add", "Cargo.toml", "Cargo.lock"])
        .spawn();
}

fn git_commit(version: &str) {
    let version = &format!("'v{version}'");
    let _ = Command::new("git").args(["commit", "-m", version]).spawn();
}
fn git_tag(version: &str) {
    let version = &format!("v{version}");
    let commit_message = &format!("'v{version}'");
    let _ = Command::new("git")
        .args(["tag", "-a", version, "-m", commit_message])
        .spawn();
}
