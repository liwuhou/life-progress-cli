// Windows is excluded: `dirs::home_dir()` resolves the profile folder through
// `SHGetKnownFolderPath` there and ignores `HOME`, so the isolated `HOME` these
// tests pass to the child process would not redirect the config directory and
// they would read the runner's real profile instead.
#![cfg(not(windows))]

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

fn cli_config_path(home: &Path) -> PathBuf {
    home.join(".config/life_progress/cli.toml")
}

fn write_cli_config(home: &Path, content: &str) {
    let path = cli_config_path(home);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn write_profile(home: &Path, content: &str) {
    let path = profile_path(home);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn write_expectancy_cache(home: &Path) {
    let cache_path = home.join(".config/life_progress/.tmp_expectancy.json");
    let parent = cache_path.parent().unwrap();
    fs::create_dir_all(parent).unwrap();
    fs::write(
        cache_path,
        r#"{"China":{"all":77.72,"male":75.0,"female":80.7}}"#,
    )
    .unwrap();
}

fn run_without_arguments(home: &Path) -> std::process::Output {
    run_with_arguments(home, &[])
}

fn run_with_arguments(home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_life-progress-cli"))
        .env("HOME", home)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn documents_display_settings_in_help() {
    let home = temporary_home("help");
    fs::create_dir_all(&home).unwrap();

    let output = run_with_arguments(&home, &["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("~/.config/life_progress/cli.toml"));
    assert!(stdout.contains("show_born = false"));
    assert!(stdout.contains("mode = 'text'"));
    assert!(stdout.contains("{elapsed}"));
    fs::remove_dir_all(home).unwrap();
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
    assert!(String::from_utf8_lossy(&output.stdout).contains("LIFE PROGRESS"));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn hides_configured_progress_fields() {
    let home = temporary_home("hidden-fields");
    write_profile(
        &home,
        "schema_version = 1\nbirthday = '1994-12-10'\ngender = 'male'\nnation = \"People's Republic of China\"\n",
    );
    write_expectancy_cache(&home);
    write_cli_config(
        &home,
        "[display]\nshow_born = false\nshow_nation = false\nshow_gender = false\nshow_elapsed = false\nshow_remaining = false\nshow_complete = false\nshow_timeline = false\n",
    );

    let output = run_without_arguments(&home);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(!stdout.contains("\n  Born"));
    assert!(!stdout.contains("\n  Nation"));
    assert!(!stdout.contains("\n  Gender"));
    assert!(!stdout.contains("\n  Elapsed"));
    assert!(!stdout.contains("\n  Remaining"));
    assert!(!stdout.contains("\n  Complete"));
    assert!(!stdout.contains("\n  Timeline"));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn renders_a_custom_text_template() {
    let home = temporary_home("text-template");
    write_profile(
        &home,
        "schema_version = 1\nbirthday = '1994-12-10'\ngender = 'male'\nnation = \"People's Republic of China\"\n",
    );
    write_expectancy_cache(&home);
    write_cli_config(
        &home,
        "[display]\nmode = 'text'\ntext_template = 'born={born}; nation={nation}; gender={gender}; elapsed={elapsed}; remaining={remaining}; complete={complete}; timeline={timeline}'\n",
    );

    let output = run_without_arguments(&home);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.starts_with("born=1994-12-10"));
    assert!(stdout.contains("; nation=People's Republic of China"));
    assert!(stdout.contains("; gender=Male"));
    assert!(stdout.contains("; elapsed="));
    assert!(stdout.contains("; remaining="));
    assert!(stdout.contains("; complete="));
    assert!(stdout.contains("; timeline="));
    assert!(!stdout.contains('{'));
    assert!(!stdout.contains('}'));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn rejects_malformed_cli_config() {
    let home = temporary_home("malformed-cli-config");
    write_cli_config(&home, "[display\nshow_born = false\n");

    let output = run_without_arguments(&home);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("parse CLI config TOML"));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn explains_how_to_recover_from_no_matches() {
    let home = temporary_home("no-matches");
    write_expectancy_cache(&home);

    let output = run_with_arguments(&home, &["search", "zzzzz"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success());
    assert!(stderr.contains("No nation matched \"zzzzz\""));
    assert!(stderr.contains("life-progress-cli search korea"));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn reports_a_missing_profile_without_creating_one() {
    let home = temporary_home("missing-profile");
    fs::create_dir_all(&home).unwrap();

    let output = run_without_arguments(&home);

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No profile configured"));
    assert!(stderr.contains("~/.config/life_progress/profile.toml"));
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
