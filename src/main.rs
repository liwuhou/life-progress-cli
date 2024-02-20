use anyhow::Result;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use colored::*;
use life_progress_core::{
    get_birthday_time, get_progress_info, search_nation, view_nation, Gender, ProgressInfo,
};
use lifespan_crawler::CountryInfo;
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Birthday, eg: 2024-02-20|20240220|unix_timestamp
    #[arg(short, long, value_parser = get_birthday_time)]
    birthday: Option<NaiveDate>,

    /// Custom config file path
    #[arg(short, long, value_parser = get_absolute_path)]
    config: Option<PathBuf>,

    /// Gender, eg: 0: female, 1: male
    #[arg(short, long, value_parser = gender_parser)]
    gender: Option<Gender>,

    /// Nation, use search command to find your nation
    #[arg(short, long, value_parser = fuzzy_search_nation_parser)]
    nation: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Search the country in the list
    Search(Search),

    /// View the country detail
    View(Search),

    /// Custom config file path
    Config(Path),
}

#[derive(Debug, Parser)]
pub struct Search {
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct Path {
    pub path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(config_path) = cli.config.as_deref() {
        println!("{}", config_path.is_absolute());
        // println!("{:?}", config_path);
        // let content = fs::read_to_string(config_path)?;
        // println!("{}", content);
    } else if let Some(birthday) = cli.birthday {
        let gender = cli.gender;
        let nation = cli.nation.as_deref();

        let ProgressInfo {
            spent,
            rest,
            progress,
            ..
        } = get_progress_info(birthday, gender, nation)?;
        println!("You spent {spent} days, completed {progress}% of life progress, still have {rest} days left. enjoy!")
    } else {
        match cli.command {
            Some(Commands::Search(Search { name })) => {
                let searched_list = search_nation(&name)?;
                if !searched_list.is_empty() {
                    for ((nation, country_info), (_, indices)) in searched_list {
                        print_nation(&nation, indices);
                        print!(" ");
                        print_country(&country_info);
                    }
                } else {
                    print_notion_error();
                }
            }
            Some(Commands::View(Search { name })) => {
                if let Some(result) = view_nation(&name) {
                    print!("{} ", name);
                    print_country(&result);
                } else {
                    print_notion_error();
                }
            }
            Some(Commands::Config(Path { path })) => {
                let result = read_config(&path);
            }
            _ => (),
        }
    }

    Ok(())
}

fn read_config(path: &str) -> Result<()> {
    let config_path = get_absolute_path(path)?;

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

fn fuzzy_search_nation_parser(input: &str) -> Result<String> {
    if input.is_empty() {
        return Ok("Common".to_string());
    }

    let searched_list = search_nation(input)?;
    let target = searched_list
        .iter()
        .max_by(|(_, (score_a, _)), (_, (score_b, _))| score_a.cmp(score_b));

    if let Some(((country_name, _), _)) = target {
        Ok(country_name.clone())
    } else {
        Ok(String::from("Common"))
    }
}

fn print_notion_error() {
    print!("{} ", String::from(" ERROR ").white().on_red());
    println!("{}", "Nothing to found");
}

fn print_nation(nation: &str, indices: Vec<usize>) {
    for (idx, char) in nation.char_indices() {
        if indices.contains(&idx) {
            print!("{}", String::from(char).green().bold());
        } else {
            print!("{}", String::from(char).bold());
        }
    }
}

fn print_country(country_info: &CountryInfo) {
    let CountryInfo { all, male, female } = country_info;
    println!("{{ Average: {}, Male: {}, Female: {} }}", all, male, female);
}
