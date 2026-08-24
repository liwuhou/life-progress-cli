use anyhow::Result;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use colored::*;
use life_progress_core::{
    get_birthday_time, get_progress_info, load_profile, search_nation, view_nation, Gender,
    ProgressInfo,
};
use lifespan_crawler::CountryInfo;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Birthday, eg: 2024-02-20|20240220|unix_timestamp
    #[arg(short, long, value_parser = get_birthday_time)]
    birthday: Option<NaiveDate>,

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
}

#[derive(Debug, Parser)]
pub struct Search {
    pub name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(birthday) = cli.birthday {
        print_progress(birthday, cli.gender, cli.nation.as_deref())?;
        return Ok(());
    }

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
        None => match load_profile()? {
            Some(profile) => {
                let birthday = get_birthday_time(&profile.birthday)?;
                print_progress(birthday, profile.gender, Some(&profile.nation))?;
            }
            None => eprintln!(
                "No profile configured at ~/.config/live_progress/profile.toml; provide --birthday instead."
            ),
        },
    }

    Ok(())
}

fn print_progress(birthday: NaiveDate, gender: Option<Gender>, nation: Option<&str>) -> Result<()> {
    let ProgressInfo {
        spent,
        rest,
        progress,
        ..
    } = get_progress_info(birthday, gender, nation)?;
    println!(
        "You spent {} days, completed {:.2}% of life progress, still have {} days left. enjoy!",
        spent, progress, rest
    );
    Ok(())
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
