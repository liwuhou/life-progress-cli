use anyhow::Result;
use chrono::NaiveDate;
use life_progress_core::{get_brithday_time, Gender};
use lifespan_crawler::get_data;
use std::env;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Birthday
    #[arg(short, long, value_parser = get_brithday_time)]
    birthday: Option<NaiveDate>,

    /// Custom config file path
    #[arg(short, long, value_parser = get_absolute_path)]
    config: Option<PathBuf>,

    /// Gender
    #[arg(short, long, value_parser = gender_parser)]
    gender: Option<Gender>,

    /// Nation
    #[arg(short, long, value_parser = fuzzly_parser)]
    nation: Option<String>,
    // Command
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(config_path) = cli.config.as_deref() {
        println!("{}", config_path.is_absolute());
        // println!("{:?}", config_path);
        // let content = fs::read_to_string(config_path)?;
        // println!("{}", content);
    } else {
        let birthday = cli.birthday;
        let gender = cli.gender;
    }
    Ok(())
}

fn get_absolute_path(path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Ok(path)
    } else {
        let mut pwd = env::current_dir()?;
        pwd.push(path);
        Ok(pwd)
    }
}

fn gender_parser(input: &str) -> Result<Gender> {
    if input != '0'.to_string() {
        Ok(Gender::Male)
    } else {
        Ok(Gender::Female)
    }
}

fn fuzzly_parser(input: &str) -> Result<String, String> {
    let data = get_data().unwrap();
    if !data.contains_key(input) {
        return Err("error".into());
    }
    if input.is_empty() {
        return Ok("Common".to_string());
    }

    let countries_list: Vec<String> = data.into_keys().collect();

    Ok(String::new())
}
