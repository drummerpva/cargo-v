use cargo_v::{read_file, update_version, update_version_by_label, VersionLabel};
use std::{
    env,
    error::Error,
    fs, io,
    process::{self, Command},
};
fn main() {
    let mut args = env::args().skip(2);
    let version = match args.next() {
        Some(v) => v,
        None => handle_error(String::from("Version not provided! You must pass the version(patch, minor, major or specific version v1.0.0 by Example)")),
    };
    let file_content = match read_file("./Cargo.toml") {
        Ok(data) => data,
        Err(err) => handle_error(format!("Can not load file: {err}")),
    };

    let (new_file_content, new_version) = match version.as_str().trim() {
        "patch" => {
            update_version_by_label(file_content, VersionLabel::Patch).unwrap_or_else(|error| {
                handle_error(error.to_string());
            })
        }
        "minor" => {
            update_version_by_label(file_content, VersionLabel::Minor).unwrap_or_else(|error| {
                handle_error(error.to_string());
            })
        }
        "major" => {
            update_version_by_label(file_content, VersionLabel::Major).unwrap_or_else(|error| {
                handle_error(error.to_string());
            })
        }
        _ => update_version(file_content, String::from(version.trim())).unwrap_or_else(|error| {
            handle_error(error.to_string());
        }),
    };
    if let Err(error) = save_new_version_in_cargo_toml(new_file_content) {
        handle_error(format!("Erro on Save new content at Cargo.toml: {error}"));
    }
    if let Err(error) = run_build() {
        handle_error(format!("Error on build: {}", error));
    }
    if let Err(error) = git_add() {
        handle_error(format!("Error at git_add: {error}"));
    }
    if let Err(error) = git_commit(&new_version) {
        handle_error(format!("Error at git_commit: {error}"));
    }
    if let Err(error) = git_tag(&new_version) {
        handle_error(format!("Error at git_tag: {error}"));
    }
    process::exit(0);
}

fn handle_error(error: String) -> ! {
    eprintln!("ERROR: {error}");
    process::exit(1);
}
fn save_new_version_in_cargo_toml(new_file_content: String) -> io::Result<()> {
    fs::write("./Cargo.toml", new_file_content)?;
    Ok(())
}
fn run_build() -> Result<(), Box<dyn Error>> {
    let _ = Command::new("cargo")
        .args(["build", "--release"])
        .output()?;
    Ok(())
}

fn git_add() -> io::Result<()> {
    let _ = Command::new("git")
        .args(["add", "Cargo.toml", "Cargo.lock"])
        .output()?;
    Ok(())
}

fn git_commit(version: &str) -> io::Result<()> {
    let version = &format!("'v{version}'");
    let _ = Command::new("git")
        .args(["commit", "-m", version])
        .output()?;
    Ok(())
}
fn git_tag(version: &str) -> io::Result<()> {
    let version = &format!("v{version}");
    let commit_message = &format!("'v{version}'");
    let _ = Command::new("git")
        .args(["tag", "-a", version, "-m", commit_message])
        .output();
    Ok(())
}
