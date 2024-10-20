# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.10](https://github.com/hadronomy/bootleg/compare/v0.1.9...v0.1.10)

### ⚙️ Miscellaneous Tasks

- Add mergify config - ([30ec826](https://github.com/hadronomy/bootleg/commit/30ec826cb866d79de5fe13cdd72d88875750ee26))
- Add dependabot config - ([81b2fc4](https://github.com/hadronomy/bootleg/commit/81b2fc4181a373b2c65f739860b0a91800befe92))
- Fix link in `README` - ([7dad7d4](https://github.com/hadronomy/bootleg/commit/7dad7d41dc9eb7e29d3e0dde5114e12f95780341))
- Add header to `README` - ([154c700](https://github.com/hadronomy/bootleg/commit/154c70055f2985978f17684ebfdc5a52a141580e))

## [0.1.9](https://github.com/hadronomy/bootleg/compare/v0.1.8...v0.1.9)

### ⚙️ Miscellaneous Tasks

- Update `shadow-rs` to `0.35.0` - ([84366a1](https://github.com/hadronomy/bootleg/commit/84366a127eed84106db8f095346e0637355d623c))
- Update dependencies - ([a889ea0](https://github.com/hadronomy/bootleg/commit/a889ea0f6c33fe5de5d2e5cfd16bd54f14da8f7b))

## [0.1.8](https://github.com/hadronomy/bootleg/compare/v0.1.7...v0.1.8)

### ⚙️ Miscellaneous Tasks

- Fix changelog header - ([d8f7484](https://github.com/hadronomy/bootleg/commit/d8f74840f41f34e01112fedb8e97da2d0788b27a))
- Improve changelog format - ([b8adcd8](https://github.com/hadronomy/bootleg/commit/b8adcd8dadd0832d743559d6b974d48b22a47cb0))

## [0.1.7](https://github.com/hadronomy/bootleg/compare/v0.1.6...v0.1.7) - 2024-09-04

### Fixed
- program not exiting when help message shown

### Other
- add `rust-analyzer` to toolchain

## [0.1.6](https://github.com/hadronomy/bootleg/compare/v0.1.5...v0.1.6) - 2024-09-03

### Added

- print the text on the clipboard when running without arguments

### Other

- remove `x86_64-apple-darwin` target
- fix formatting
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
