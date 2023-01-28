use std::process;

pub enum VersionLabel {
    Patch,
    Minor,
    Major,
}

pub fn update_version_by_label(
    cargo_toml_content: String,
    version: VersionLabel,
) -> (String, String) {
    let old_version = get_version(&cargo_toml_content);
    let (major, minor, patch) = get_version_as_tuple(&old_version);
    update_version(
        cargo_toml_content,
        match version {
            VersionLabel::Patch => {
                format!("{major}.{minor}.{}", increment_version(patch))
            }
            VersionLabel::Minor => {
                format!("{major}.{}.0", increment_version(minor))
            }
            VersionLabel::Major => format!("{}.0.0", increment_version(major)),
        },
    )
}

pub fn update_version(cargo_toml_content: String, version: String) -> (String, String) {
    let version = version.replace("v", "");
    let old_version = get_version(&cargo_toml_content);
    verify_new_version_is_grather(&old_version, &version);
    (cargo_toml_content.replace(&old_version, &version), version)
}
fn verify_new_version_is_grather(old_version: &str, new_version: &str) {
    let (old_major, old_minor, old_patch) = get_version_as_tuple(old_version);
    let (new_major, new_minor, new_patch) = get_version_as_tuple(new_version);
    let old_major: usize = old_major.parse().unwrap();
    let old_minor: usize = old_minor.parse().unwrap();
    let old_patch: usize = old_patch.parse().unwrap();
    let new_major: usize = new_major.parse().unwrap();
    let new_minor: usize = new_minor.parse().unwrap();
    let new_patch: usize = new_patch.parse().unwrap();
    if old_major != new_major {
        if new_major < old_major {
            panic!("You can not set a version lower than the current version");
        };
        if new_minor != 0 || new_patch != 0 {
            panic!("You can not set a version lower than the current version");
        }
        return;
    }
    if old_minor != new_minor {
        if new_minor < old_minor {
            panic!("You can not set a version lower than the current version");
        }
        if new_patch != 0 {
            panic!("You can not set a version lower than the current version");
        }
        return;
    }
    if new_patch < old_patch {
        panic!("You can not set a version lower than the current version");
    }
    if new_patch == old_patch {
        panic!("You can not set a version lower than the current version");
    }
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
fn increment_version(single_version: &str) -> usize {
    single_version.parse::<usize>().unwrap() + 1
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_get_version() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version = get_version(&input);
        assert_eq!(version, String::from("0.0.1"));
    }
    #[test]
    fn should_get_version_tuple() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_string = get_version(&input);
        let version = get_version_as_tuple(&version_string);
        assert_eq!(version, ("0", "0", "1"));
    }

    #[test]
    fn should_update_project_version_by_hand() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.0.2");

        assert_eq!(
            update_version(input, version_expected.clone()),
            (expected, version_expected)
        );
    }

    #[test]
    fn should_update_project_version_patch() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.0.2");
        assert_eq!(
            update_version_by_label(input, VersionLabel::Patch),
            (expected, version_expected)
        );
    }
    #[test]
    fn should_update_project_version_minor() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.1.0");

        assert_eq!(
            update_version_by_label(input, VersionLabel::Minor),
            (expected, version_expected)
        );
    }
    #[test]
    fn should_update_project_version_major() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"1.0.0\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("1.0.0");

        assert_eq!(
            update_version_by_label(input, VersionLabel::Major),
            (expected, version_expected)
        );
    }

    #[test]
    #[should_panic(expected = "You can not set a version lower than the current version")]
    fn should_panic_on_version_patch_passed_lower_than_current() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        update_version(input, "0.0.1".into());
    }
    #[test]
    #[should_panic(expected = "You can not set a version lower than the current version")]
    fn should_panic_on_version_minor_passed_lower_than_current() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.2.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        update_version(input, "0.1.0".into());
    }
    #[test]
    #[should_panic(expected = "You can not set a version lower than the current version")]
    fn should_panic_on_version_major_passed_lower_than_current() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        update_version(input, "1.0.0".into());
    }
    #[test]
    #[should_panic(expected = "InvalidDigit")]
    fn should_panic_on_version_passed_had_negative_number() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        update_version(input, "-2.1.0".into());
    }

    #[test]
    fn should_acept_v_prefix() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.0.2");

        assert_eq!(
            update_version(input, "v0.0.2".into()),
            (expected, version_expected)
        );
    }
}
