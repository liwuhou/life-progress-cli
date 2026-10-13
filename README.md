# life-progress-cli

## Installation

### GitHub Releases

Download a prebuilt binary from the [releases page](https://github.com/liwuhou/life-progress-cli/releases), with its SHA-256 checksum. Released assets cover Linux x86_64, macOS arm64, macOS Intel (x86_64), and Windows x86_64.

Windows users can download `life-progress-cli-windows-x86_64.exe` and install it from PowerShell:

```powershell
New-Item -ItemType Directory -Force "$HOME\bin"
Invoke-WebRequest `
  https://github.com/liwuhou/life-progress-cli/releases/latest/download/life-progress-cli-windows-x86_64.exe `
  -OutFile "$HOME\bin\life-progress-cli.exe"
```

Add `$HOME\bin` to `PATH` if it is not already available.

### Cargo

Install directly from this repository on macOS, Linux, or Windows (requires Rust and Cargo):

```shell
cargo install --git https://github.com/liwuhou/life-progress-cli.git --locked
```

The package is prepared for a future crates.io release; after it is published, the shorter command will be:

```shell
cargo install life-progress-cli
```

### Homebrew

An official Homebrew tap is not published yet. Until a tap is available, macOS users can use the GitHub release binary or the Cargo installation above.

```shell
Usage: life-progress-cli [OPTIONS] [COMMAND]

Commands:
  search  Search the country in the list
  view    View the country detail
  help    Print this message or the help of the given subcommand(s)

Options:
  -b, --birthday <BIRTHDAY>  Birthday, eg: 2024-02-20|20240220|unix_timestamp
  -g, --gender <GENDER>      Gender, eg: 0: female, 1: male
  -n, --nation <NATION>      Nation, use search command to find your nation
  -h, --help                 Print help
  -V, --version              Print version
```

When invoked without a birthday or subcommand, the CLI loads `~/.config/life_progress/profile.toml`. The profile must use the shared v1 schema:

```toml
schema_version = 1
birthday = "1994-12-10"
gender = "male" # optional; "male" or "female"
nation = "People's Republic of China"
```

Malformed profiles, future birthdays, non-canonical nations, unknown fields, and unsupported schema versions are rejected. Use `-b`, `-g`, and `-n` to override the profile for one invocation.

Display fields are configured separately in `~/.config/life_progress/cli.toml`; this keeps the shared profile contract portable:

```toml
[display]
show_born = false
show_nation = false
show_gender = false
show_elapsed = true
show_remaining = true
show_complete = true
show_timeline = false
```

All seven fields default to visible. To render only plain text, select `mode = "text"` and define `text_template`:

```toml
[display]
mode = "text"
text_template = "{elapsed} days elapsed; {remaining} days remaining ({complete})"
```

Supported placeholders are `{born}`, `{nation}`, `{gender}`, `{elapsed}`, `{remaining}`, `{complete}`, and `{timeline}`.


Sample Example

```shell
$ life-progress-cli -b 2020-10-24 -g 1 -n China

LIFE PROGRESS
────────────────────────────────────────
  Born         2020-10-24
  Nation       China
  Gender       Male
  Elapsed      2132 days
  Remaining    25316 days
  Complete     7.77%
  Timeline     ██░░░░░░░░░░░░░░░░░░░░░░
────────────────────────────────────────
  Keep moving.
```

Use `life-progress-cli search` to find a country in the list.

```shell
$ life-progress-cli search korea

NATION SEARCH
  korea
────────────────────────────────────────
  South Korea
    Average  84.33 years  Male 81.19 years  Female 87.16 years

  North Korea
    Average  73.64 years  Male 71.46 years  Female 75.74 years
```

