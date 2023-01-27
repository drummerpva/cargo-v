use std::process;

pub fn update_version(cargo_toml_content: String, version: String) -> String {
    let old_version = get_version(&cargo_toml_content);
    cargo_toml_content.replace(&old_version, &version)
}

fn get_version(cargo_toml_content: &str) -> String {
    cargo_toml_content
        .lines()
        .find(|line| line.contains("version"))
        .unwrap_or_else(|| {
            eprintln!("Cargo.toml don't have a version tag");
            process::exit(1);
        })
        .split("=")
        .last()
        .unwrap()
        .replace("\"", "")
        .trim()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_update_project_version() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");

        assert_eq!(update_version(input, "0.0.2".into()), expected);
    }
}
