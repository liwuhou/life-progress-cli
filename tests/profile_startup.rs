use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temporary_home(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "life-progress-cli-{name}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn profile_path(home: &Path) -> PathBuf {
    home.join(".config/life_progress/profile.toml")
}

fn write_profile(home: &Path, content: &str) {
    let path = profile_path(home);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn write_expectancy_cache(home: &Path) {
    let cache_path = home.join(".config/life_progress/.tmp_expectancy.json");
    fs::write(
        cache_path,
        r#"{"China":{"all":77.72,"male":75.0,"female":80.7}}"#,
    )
    .unwrap();
}

fn run_without_arguments(home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_life-progress-cli"))
        .env("HOME", home)
        .output()
        .unwrap()
}

#[test]
fn starts_with_a_canonical_profile_against_china_source_data() {
    let home = temporary_home("valid-profile");
    write_profile(
        &home,
        "schema_version = 1\nbirthday = '1994-12-10'\ngender = 'male'\nnation = \"People's Republic of China\"\n",
    );
    write_expectancy_cache(&home);

    let output = run_without_arguments(&home);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("You spent"));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn reports_a_missing_profile_without_creating_one() {
    let home = temporary_home("missing-profile");
    fs::create_dir_all(&home).unwrap();

    let output = run_without_arguments(&home);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("No profile configured"));
    assert!(!profile_path(&home).exists());
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn fails_to_start_with_an_invalid_profile() {
    let home = temporary_home("invalid-profile");
    write_profile(
        &home,
        "schema_version = 1\nbirthday = '1994-12-10'\nnation = [\n",
    );

    let output = run_without_arguments(&home);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("parse profile TOML"));
    fs::remove_dir_all(home).unwrap();
}
