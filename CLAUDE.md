# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build              # Build the project
cargo run --             # Run with default options
cargo run -- -b 2020-10-24                    # Run with birthday
cargo run -- -b 2020-10-24 -g 1 -n "China"    # With gender (0/1) and nation
cargo run -- search korea                     # Search for a country
cargo run -- view "South Korea"               # View country life expectancy
```

Running `cargo run --` without a birthday or subcommand reads the shared profile at `~/.config/live_progress/profile.toml`. Its v1 TOML fields are `schema_version = 1`, `birthday` (`YYYY-MM-DD`), optional lowercase `gender`, and canonical `nation`.

## Build Universal Binary (macOS)

Build a universal binary that works on both Intel (x86_64) and Apple Silicon (arm64) Macs:

```bash
make build-universal     # Build universal binary to target/universal/
make install-universal   # Install to ~/.cargo/bin/
```

Requires `aarch64-apple-darwin` target: `rustup target add aarch64-apple-darwin`

## Project Structure

This is a Rust CLI application with a workspace containing 3 crates:

- **life-progress-cli** (root) - Main CLI binary using `clap` for argument parsing
- **lifespan-crawler** - Fetches life expectancy data from Wikipedia, caches to `~/.config/live_progress/.tmp_expectancy.json`
- **life-progress-core** - Core business logic for calculating life progress, fuzzy country search

## Architecture

```
┌─────────────────────┐
│   main.rs (CLI)     │ Uses clap for parsing
└─────────┬───────────┘
          │
    ┌─────┴──────┐
    │            │
┌───▼────┐  ┌────▼──────────┐
│ core   │  │ lifespan-     │
│ (logic)│  │ crawler       │
└────────┘  │ (Wikipedia)   │
            └───────────────┘
```

Data flow:
1. `lifespan-crawler` fetches life expectancy from Wikipedia → caches as JSON
2. `life-progress-core` calculates progress based on birthday, gender, nation
3. `main.rs` handles CLI output with colored formatting

Key types:
- `CountryInfo { all, male, female }` - Life expectancy in years
- `ProgressInfo { spent, progress, rest, rest_progress }` - Calculation results
- `Gender { Male, Female }` - Gender enum

Birthday formats supported: `YYYY-MM-DD`, `YYYYMMDD`, Unix milliseconds
