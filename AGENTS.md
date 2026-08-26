# Repository Guidelines

## Project Overview

`life-progress-cli` is a Rust CLI that estimates life progress from a birthday, gender, and country life-expectancy data. It is a Cargo workspace: the root binary delegates domain calculations to `life-progress-core`, which obtains cached or fetched country data through `lifespan-crawler`.

## Architecture & Data Flow

`src/main.rs` is the Clap entry point. Explicit birthday options take precedence over subcommands; without a birthday or subcommand, it loads the shared profile from `~/.config/life_progress/profile.toml`.

```text
CLI (src/main.rs)
  -> life-progress-core (parse date, resolve country, calculate progress)
    -> lifespan-crawler (cache -> blocking Wikipedia fetch -> bundled fallback JSON)
```

- `crates/life-progress-core/src/lib.rs` exposes the profile contract (`Profile`, `load_profile`), domain types, progress calculation, and country lookup APIs.
- `crates/lifespan-crawler/src/lib.rs` is synchronous and cache-first. It reads `~/.config/life_progress/.tmp_expectancy.json`; a miss fetches Wikipedia and persists JSON. Fetch failure uses bundled default expectancy data.
- Country selection uses fuzzy matching and falls back to `Common`; `view_nation` performs an exact lookup and maps data failures to `None`.
- A shared profile is versioned TOML (`schema_version = 1`, `birthday`, optional lowercase `gender`, canonical `nation`). Loading treats missing profiles separately and rejects malformed or semantically invalid data. Preserve the canonical `People's Republic of China` profile name even when fetched data uses the `China` source key.
- CLI-only display preferences live in optional `~/.config/life_progress/cli.toml` under `[display]`; all seven report fields default visible, can be hidden independently, and `mode = "text"` enables `text_template` placeholders. Keep these settings out of the shared profile schema.

## Key Directories

- `src/` — root CLI binary and output formatting.
- `crates/life-progress-core/` — date parsing, country selection, and progress calculations.
- `crates/lifespan-crawler/` — data model, cache/network retrieval, HTML parsing, bundled fallback data.
- `target/` — generated Cargo artifacts; never edit or commit.

## Development Commands

Run commands from the repository root:

```bash
cargo build
cargo run --                         # reads ~/.config/life_progress/profile.toml
cargo run -- -b 2020-10-24 -g 1 -n "China"
cargo run -- search korea
cargo run -- view "South Korea"
```

macOS universal release build (requires the Apple Silicon target):

```bash
rustup target add aarch64-apple-darwin
make build-universal       # output: target/universal/life-progress-cli
make install-universal     # installs to ~/.cargo/bin
```


Pushing a `v*` tag triggers `.github/workflows/release.yml`; it tests and formats the workspace, builds Linux x86_64 and macOS arm64 binaries, creates SHA-256 files, and publishes a GitHub release with generated notes.
`make release-universal` aliases the universal build. `make clean` removes Cargo artifacts and the universal output.

## Code Conventions & Common Patterns

- Use Rust 2021 idioms and `anyhow::Result` with `?` for ordinary fallible operations; do not introduce a separate error framework.
- Keep the existing synchronous model: crawler networking uses blocking `reqwest`; there is no async runtime or dependency-injection layer.
- Domain types use PascalCase (`Gender`, `ProgressInfo`, `CountryInfo`); functions and fields use snake_case (`get_progress_info`, `rest_progress`).
- Keep business logic in `life-progress-core`; keep cache, fetch, parse, and persistence behavior in `lifespan-crawler`; keep Clap parsing and terminal rendering in `src/main.rs`.
- Preserve current behavior deliberately when changing validation: future birthdays and invalid millisecond timestamps currently panic, while country lookup can fall back to `Common` and `view_nation` returns `None` for unavailable data.
- Use colocated `#[cfg(test)] mod tests` for core unit tests. Existing tests return `anyhow::Result<()>` and use direct assertions.

## Important Files

- `Cargo.toml` — root package plus workspace membership and local crate dependencies.
- `Cargo.lock` — committed dependency resolution; update it with dependency changes.
- `src/main.rs` — executable entry point, Clap definitions, command dispatch, CLI output.
- `~/.config/life_progress/cli.toml` — optional CLI display visibility settings; defaults keep all progress fields visible.
- `crates/life-progress-core/src/lib.rs` — shared profile validation/loading, domain API, and inline unit tests.
- `crates/lifespan-crawler/src/lib.rs` — country data retrieval and fallback logic.
- `crates/lifespan-crawler/default_expectancy.json` — offline fallback data.
- `Makefile` — macOS universal build and install targets.
- `.github/workflows/release.yml` — tag-triggered cross-platform build and GitHub release pipeline.
- `README.md` and `CLAUDE.md` — CLI usage and contributor examples.

## Runtime/Tooling Preferences

- Use Cargo/Rust, not Node or Bun tooling. The workspace targets Rust edition 2021; no `rust-toolchain` file pins a compiler version.
- Treat `Cargo.lock` as the package-manager lockfile. The two workspace crates are Git submodules (`.gitmodules`); initialize them before building in a fresh clone.
- CI/release automation is defined in `.github/workflows/release.yml`; standard `cargo fmt`, `cargo clippy`, and `cargo check` remain optional local checks.

## Testing & QA

- Core unit tests are colocated in `crates/life-progress-core/src/lib.rs`; CLI startup coverage is in `tests/profile_startup.rs` and executes the built binary with isolated home directories.
- Run the full available suite with `cargo test`. There is no configured coverage tool or coverage threshold.
- For CLI-affecting work, add or update isolated-home integration coverage when changing profile startup behavior.
