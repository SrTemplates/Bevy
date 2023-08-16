# Kill Errors - 404

<p align="center">
    <img alt="GitHub CI Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/ci.yml?label=ci&style=flat-square">
    <img alt="GitHub Build Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/build.yml?label=Build%20Native&style=flat-square">
    <img alt="GitHub Android Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/build-android.yml?label=Build%20Android&style=flat-square">
    <a href="https://sergioribera.github.io/game_kill_errors"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/game_kill_errors/release-gh-pages.yml?label=Build%20Web&style=flat-square"></a>
    <a href="https://github.com/SergioRibera/game_kill_errors/releases"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/SergioRibera/game_kill_errors?label=download&style=flat-square"></a>
</p>

# Support Platforms
- Native (MacOs, Linux & Windows)
- Web (Wasm)
- Library (Usable in other rust proyects)
- Mobile
  - Android
  - iOS (⚠️ Soon)

# Use this Template
> This require `--allow-commands` to create hooks for git

# Requirements
- Rust
- Cargo
- [Cargo Make](https://github.com/sagiegurari/cargo-make)
- [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
- [Trunk](https://trunkrs.dev) (Optional for web development)

# Development Guide
- Edit the `.env` file if you need
- Edit `src` folder
- Run `cargo make dev` for run as development mode (Native window)
- Run `cargo make --list-all-steps` for check all aviable tasks

#### Other CargoMake Tasks

* **check** - Check all issues, format and code quality
* **clean** - Clean all target directory
* **clippy** - Check code quality
* **default** - Check all issues, format and code quality
* **dev** - Run native launcher with development configuration
* **fix-all** - Try fix all clippy and format issues
* **fix-fmt** - Fix format
* **fmt** - Check format quality
* **test** - Check all unit test
