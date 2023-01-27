use std::process;

pub enum VersionLabel {
    Patch,
    Minor,
    Major,
}

pub fn update_version_by_label(cargo_toml_content: String, version: VersionLabel) -> String {
    let old_version = get_version(&cargo_toml_content);
    let (major, minor, patch) = get_version_as_tuple(&old_version);
    update_version(
        cargo_toml_content,
        match version {
            VersionLabel::Patch => format!(
                "{}.{}.{}",
                major,
                minor,
                patch.parse::<usize>().unwrap() + 1
            ),
            VersionLabel::Minor => {
                format!("{}.{}.{}", major, minor.parse::<usize>().unwrap() + 1, 0)
            }
            VersionLabel::Major => format!("{}.{}.{}", major.parse::<usize>().unwrap() + 1, 0, 0),
        },
    )
}

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
        .split('=')
        .last()
        .unwrap()
        .replace('\"', "")
        .trim()
        .to_string()
}
fn get_version_as_tuple(version: &str) -> (&str, &str, &str) {
    let vec: Vec<&str> = version.split('.').collect();
    (vec[0], vec[1], vec[2])
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

    #[test]
    fn should_update_project_version_patch() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");

        assert_eq!(
            update_version_by_label(input, VersionLabel::Patch),
            expected
        );
    }
    #[test]
    fn should_update_project_version_minor() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");

        assert_eq!(
            update_version_by_label(input, VersionLabel::Minor),
            expected
        );
    }
    #[test]
    fn should_update_project_version_major() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"1.0.0\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");

        assert_eq!(
            update_version_by_label(input, VersionLabel::Major),
            expected
        );
    }
}
