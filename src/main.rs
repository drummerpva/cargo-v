use cargo_v::{
    get_version_from_args_list, read_file, save_data_in_file, update_version,
    update_version_by_label, VersionLabel,
};
use std::{
    env,
    error::Error,
    io,
    process::{self, Command},
};
fn main() {
    let args = env::args();
    let version = match get_version_from_args_list(args) {
        Ok(v) => v,
        Err(err) => handle_error(err.to_string()),
    };
    let file_name = "./Cargo.toml";
    let file_content = match read_file(file_name) {
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
    if let Err(error) = save_data_in_file(new_file_content, file_name) {
        handle_error(format!("Erro on Save new content at {file_name}: {error}"));
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
