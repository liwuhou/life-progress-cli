use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use colored::*;
use life_progress_core::{
    get_birthday_time, get_progress_info, load_profile, profile_path, search_nation, view_nation,
    Gender, ProgressInfo,
};
use lifespan_crawler::CountryInfo;
use serde::Deserialize;
use std::fs;
const DIVIDER: &str = "────────────────────────────────────────";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CliConfig {
    #[serde(default)]
    display: DisplayConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DisplayMode {
    #[default]
    Report,
    Text,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct DisplayConfig {
    mode: DisplayMode,
    text_template: String,
    show_born: bool,
    show_nation: bool,
    show_gender: bool,
    show_elapsed: bool,
    show_remaining: bool,
    show_complete: bool,
    show_timeline: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Report,
            text_template: "{elapsed} days elapsed, {remaining} days remaining ({complete})"
                .to_string(),
            show_born: true,
            show_nation: true,
            show_gender: true,
            show_elapsed: true,
            show_remaining: true,
            show_complete: true,
            show_timeline: true,
        }
    }
}

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    after_help = "Display settings (optional): ~/.config/life_progress/cli.toml\n\
                  [display]\n\
                  show_born = false\n\
                  show_nation = false\n\
                  show_gender = false\n\
                  show_elapsed = false\n\
                  show_remaining = false\n\
                  show_complete = false\n\
                  show_timeline = false\n\
                  \n\
                  Use mode = 'text' with text_template for plain text output.\n\
                  Placeholders: {born}, {nation}, {gender}, {elapsed}, {remaining}, {complete}, {timeline}."
)]
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
fn load_display_config() -> Result<DisplayConfig> {
    let path = profile_path()?
        .parent()
        .context("profile path has no parent directory")?
        .join("cli.toml");
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(DisplayConfig::default())
        }
        Err(error) => {
            return Err(error).with_context(|| format!("read CLI config at {}", path.display()))
        }
    };

    Ok(toml::from_str::<CliConfig>(&content)
        .context("parse CLI config TOML")?
        .display)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let display = load_display_config()?;

    if let Some(birthday) = cli.birthday {
        print_progress(birthday, cli.gender, cli.nation.as_deref(), &display)?;
        return Ok(());
    }

    match cli.command {
        Some(Commands::Search(Search { name })) => {
            let searched_list = search_nation(&name)?;
            println!();
            println!("{}", "NATION SEARCH".bold().cyan());
            println!("  {}", name.dimmed());
            println!("{}", DIVIDER.dimmed());

            if !searched_list.is_empty() {
                for ((nation, country_info), (_, indices)) in searched_list {
                    print!("  ");
                    print_nation(&nation, indices);
                    println!();
                    print_country(&country_info);
                    println!();
                }
            } else {
                print_nation_error(&name);
            }
        }
        Some(Commands::View(Search { name })) => {
            if let Some(result) = view_nation(&name) {
                println!();
                println!("{}", "NATION DETAIL".bold().cyan());
                println!("  {}", name);
                println!("{}", DIVIDER.dimmed());
                print_country(&result);
            } else {
                print_nation_error(&name);
            }
        }
        None => match load_profile()? {
            Some(profile) => {
                let birthday = get_birthday_time(&profile.birthday)?;
                print_progress(birthday, profile.gender, Some(&profile.nation), &display)?;
            }
            None => eprintln!(
                "No profile configured at ~/.config/life_progress/profile.toml; provide --birthday instead."
            ),
        },
    }

    Ok(())
}

fn print_progress(
    birthday: NaiveDate,
    gender: Option<Gender>,
    nation: Option<&str>,
    display: &DisplayConfig,
) -> Result<()> {
    let gender_label = match gender.as_ref() {
        Some(Gender::Male) => "Male",
        Some(Gender::Female) => "Female",
        None => "All",
    };
    let ProgressInfo {
        spent,
        rest,
        progress,
        ..
    } = get_progress_info(birthday, gender, nation)?;
    if matches!(&display.mode, DisplayMode::Text) {
        println!(
            "{}",
            render_text(
                &display.text_template,
                birthday,
                gender_label,
                nation,
                spent,
                rest,
                progress,
            )?
        );
        return Ok(());
    }

    println!();
    println!("{}", "LIFE PROGRESS".bold().cyan());
    println!("{}", DIVIDER.dimmed());
    if display.show_born {
        println!("  {:<12} {}", "Born", birthday.format("%Y-%m-%d"));
    }
    if display.show_nation {
        println!("  {:<12} {}", "Nation", nation.unwrap_or("Common"));
    }
    if display.show_gender {
        println!("  {:<12} {}", "Gender", gender_label);
    }
    if display.show_elapsed {
        println!("  {:<12} {} days", "Elapsed", spent);
    }
    if display.show_remaining {
        println!("  {:<12} {} days", "Remaining", rest);
    }
    if display.show_complete {
        println!(
            "  {:<12} {}",
            "Complete",
            format!("{:.2}%", progress).yellow().bold()
        );
    }
    if display.show_timeline {
        println!("  {:<12} {}", "Timeline", progress_bar(progress));
    }
    println!();
    println!("{}", DIVIDER.dimmed());
    println!("{}", "  Keep moving.".dimmed());
    Ok(())
}

fn render_text(
    template: &str,
    birthday: NaiveDate,
    gender: &str,
    nation: Option<&str>,
    elapsed: u64,
    remaining: u64,
    complete: f64,
) -> Result<String> {
    let born = birthday.format("%Y-%m-%d").to_string();
    let nation = nation.unwrap_or("Common");
    let elapsed = elapsed.to_string();
    let remaining = remaining.to_string();
    let timeline = plain_progress_bar(complete);
    let complete = format!("{complete:.2}%");
    let values = [
        ("born", born.as_str()),
        ("nation", nation),
        ("gender", gender),
        ("elapsed", elapsed.as_str()),
        ("remaining", remaining.as_str()),
        ("complete", complete.as_str()),
        ("timeline", timeline.as_str()),
    ];
    let mut output = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        output.push_str(&rest[..start]);
        let placeholder = &rest[start + 1..];
        let end = placeholder
            .find('}')
            .ok_or_else(|| anyhow::anyhow!("display template has an unclosed placeholder"))?;
        let key = &placeholder[..end];
        let value = values
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .ok_or_else(|| anyhow::anyhow!("unsupported display placeholder {{{key}}}"))?;
        output.push_str(value);
        rest = &placeholder[end + 1..];
    }
    output.push_str(rest);
    Ok(output)
}

fn gender_parser(input: &str) -> Result<Gender> {
    if input != '0'.to_string() {
        Ok(Gender::Male)
    } else {
        Ok(Gender::Female)
    }
}

fn progress_bar(progress: f64) -> String {
    plain_progress_bar(progress).green().to_string()
}

fn plain_progress_bar(progress: f64) -> String {
    const WIDTH: usize = 24;
    let completed = ((progress.clamp(0.0, 100.0) / 100.0) * WIDTH as f64).round() as usize;
    format!("{}{}", "█".repeat(completed), "░".repeat(WIDTH - completed))
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

fn print_nation_error(query: &str) {
    eprintln!(
        "{} {}",
        "No nation matched".red().bold(),
        format!("\"{query}\"").yellow()
    );
    eprintln!(
        "{}",
        "Try a broader name, for example: life-progress-cli search korea".dimmed()
    );
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
    println!(
        "    {:<8} {:>6.2} years  {:<8} {:>6.2} years  {:<8} {:>6.2} years",
        "Average", all, "Male", male, "Female", female
    );
}
