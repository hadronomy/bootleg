# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5](https://github.com/Hadronomy/bootleg/compare/v0.1.4...v0.1.5) - 2024-04-25

### Other
- replace `bootleg` str literals with `build::PROJECT_NAME`

## [0.1.4](https://github.com/Hadronomy/bootleg/compare/v0.1.3...v0.1.4) - 2024-04-24

### Other
- disable github releases for `release-plz`

## [0.1.3](https://github.com/Hadronomy/bootleg/compare/v0.1.2...v0.1.3) - 2024-04-24

### Other
- disable releases for `release_plz`

## [0.1.2](https://github.com/Hadronomy/bootleg/compare/v0.1.1...v0.1.2) - 2024-04-24

### Fixed
- update examples binary name to be `bootleg`

### Other
- remove `ci` workflow
- add `cargo-dist` workflow
- update help image to match
- fix placeholders not updated

## [0.1.1](https://github.com/Hadronomy/bootleg/compare/v0.1.0...v0.1.1) - 2024-04-24

### Other
- disable incremental compilation for release
- add workflow to publish binaries
- add incremental compilation
- add missing `README.md`

## [0.1.0](https://github.com/Hadronomy/bootleg/releases/tag/v0.1.0) - 2024-04-24

### Fixed
- adjust error span len to match new name

### Other
- add step to generate github token
- run `release-plz init`
- fix `Args::init_cli`
- fix `Cargo.toml` to not use workspace versions
- add `.gitignore`
- add `LICENSE` and `rust-toolchain.toml`
- rename `clip` to `bootleg`
