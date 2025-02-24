# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/hadronomy/bootleg/compare/v0.1.14...v0.2.0)

### 👷 CI/CD

- *(deps)* Bump shadow-rs from 0.38.1 to 1.0.0 (#75) - ([bd46ee3](https://github.com/hadronomy/bootleg/commit/bd46ee382f4f0c0921a77f9fd2d3eb88fd26f2b1))
- *(deps)* Bump clap from 4.5.30 to 4.5.31 (#74) - ([1c7a106](https://github.com/hadronomy/bootleg/commit/1c7a106a3eeb8da1ab7997bf7de9a78b10f6b055))
- *(deps)* Bump serde from 1.0.217 to 1.0.218 (#73) - ([ba33cd0](https://github.com/hadronomy/bootleg/commit/ba33cd03730153dd2571b9971f799b1729330478))
- *(deps)* Bump clap from 4.5.29 to 4.5.30 (#72) - ([a02c11b](https://github.com/hadronomy/bootleg/commit/a02c11bff50964da5ce20d0baeaa6ad06498dd3a))
- *(deps)* Bump clap from 4.5.28 to 4.5.29 (#71) - ([5f57006](https://github.com/hadronomy/bootleg/commit/5f57006d116b0fb8166a69c90ffc900ee2947ab3))
- *(deps)* Bump shadow-rs from 0.38.0 to 0.38.1 (#70) - ([cc81e79](https://github.com/hadronomy/bootleg/commit/cc81e791d43fb756ec7c222350c78a401e4bf690))
- *(deps)* Bump toml from 0.8.19 to 0.8.20 (#69) - ([04276d9](https://github.com/hadronomy/bootleg/commit/04276d917d1639702c18932a9be85a971daf238f))
- *(deps)* Bump clap from 4.5.27 to 4.5.28 (#68) - ([b9304c2](https://github.com/hadronomy/bootleg/commit/b9304c244dfe37d5110994251edfed6b0213140d))
- *(deps)* Bump miette from 7.4.0 to 7.5.0 (#67) - ([9d04251](https://github.com/hadronomy/bootleg/commit/9d0425156e6eae0233a4c74eba580dcca4ac3d59))
- *(deps)* Bump termimad from 0.31.1 to 0.31.2 (#66) - ([949ce63](https://github.com/hadronomy/bootleg/commit/949ce63aedbc5d20c5207c409e5373ae3f27c4d9))
- *(deps)* Bump shadow-rs from 0.37.0 to 0.38.0 (#65) - ([75c2419](https://github.com/hadronomy/bootleg/commit/75c24192189dd31d9c175a7476070111d7002736))
- *(deps)* Bump clap from 4.5.26 to 4.5.27 (#64) - ([5aea2c9](https://github.com/hadronomy/bootleg/commit/5aea2c9949c4392aa67a8ba94cf9f71350c045c9))
- *(deps)* Bump actions/attest-build-provenance from 1 to 2 (#62) - ([43f3e47](https://github.com/hadronomy/bootleg/commit/43f3e47be1388e756e3ac7085bafecc13984c18f))

## [0.1.14](https://github.com/hadronomy/bootleg/compare/v0.1.13...v0.1.14)

### 🚀 Features

- Display version information in `TOML` format - ([978acf6](https://github.com/hadronomy/bootleg/commit/978acf617576915f5262cb21c7352e0ced04d15f))

### 👷 CI/CD

- *(deps)* Bump thiserror from 2.0.10 to 2.0.11 (#61) - ([4805ae9](https://github.com/hadronomy/bootleg/commit/4805ae96691490a83e5a6404e6fbf740acd7d522))
- *(deps)* Bump clap from 4.5.24 to 4.5.26 (#59) - ([9a2f763](https://github.com/hadronomy/bootleg/commit/9a2f763a5479264152e86d9eaf273252b35e8295))
- *(deps)* Bump thiserror from 2.0.9 to 2.0.10 (#58) - ([ada4294](https://github.com/hadronomy/bootleg/commit/ada4294c0afb91224ef99f973648f5a8b206915c))
- *(deps)* Bump clap from 4.5.23 to 4.5.24 (#57) - ([ebb32c3](https://github.com/hadronomy/bootleg/commit/ebb32c37de704feae64ee8d118cbd3690b16c491))
- *(deps)* Bump actions/attest-build-provenance from 1 to 2 (#55) - ([74b195d](https://github.com/hadronomy/bootleg/commit/74b195d3255fedfe04906923ff6bbc2391c269ac))
- Fix out of date contents for `release.yml` - ([4b098e0](https://github.com/hadronomy/bootleg/commit/4b098e0741e620f918b908f282194ba472dce415))

## [0.1.13](https://github.com/hadronomy/bootleg/compare/v0.1.12...v0.1.13)

### 🚀 Features

- Add indicator to version commit if it was built with a dirty repo - ([422bc8d](https://github.com/hadronomy/bootleg/commit/422bc8d9db9f9bc547f42def2df905818a259d0d))
- Add panic messages for humans - ([d38bbb2](https://github.com/hadronomy/bootleg/commit/d38bbb22ac6c9834e00ddf89f9995e5b763c0489))
- Add wayy more information to `--version` flag - ([77a0cf6](https://github.com/hadronomy/bootleg/commit/77a0cf6218bccb165bc4d77abb8dd6342812058f))

### 🐛 Bug Fixes

- Deprecated `shadow_rs::new` - ([b13b044](https://github.com/hadronomy/bootleg/commit/b13b04470fbbd87474633d860c2bd042ee36988a))
- Fixup! chore: add `mutants.out` to `.gitignore` - ([14af37f](https://github.com/hadronomy/bootleg/commit/14af37fbb1a0fb9d04bebb6f621ab6572564fa36))

### 📦 Bumps

- Update toolchain from `1.81` to `1.83` - ([0133fae](https://github.com/hadronomy/bootleg/commit/0133fae5317e6f605ab7357af7ce0992e297b0e7))

### 🚜 Refactor

- Move print version logic to its own function - ([532f6f7](https://github.com/hadronomy/bootleg/commit/532f6f7e3c7e9cacf8d6c80b06a81d20ad0ff2d5))
- Move `main.rs` to `src/bin/bootleg.rs` - ([1f5fe3a](https://github.com/hadronomy/bootleg/commit/1f5fe3a0006daf4856836c6b63f95ee56ab51430))
- Split `main` function into multiple - ([0888f44](https://github.com/hadronomy/bootleg/commit/0888f44fc549f1a690617765842e7d8f606e1c91))

### 👷 CI/CD

- *(deps)* Bump thiserror from 2.0.8 to 2.0.9 (#53) - ([7d43a10](https://github.com/hadronomy/bootleg/commit/7d43a1024eb8e0b8921692f9a1a1dc9f5d92c3cd))
- *(deps)* Bump predicates from 3.1.2 to 3.1.3 (#52) - ([811566b](https://github.com/hadronomy/bootleg/commit/811566bae2773b0c007ffd43da013c920280645e))
- *(deps)* Bump thiserror from 2.0.7 to 2.0.8 (#51) - ([60aedde](https://github.com/hadronomy/bootleg/commit/60aedde40ff96a4d7e58ddf1e329caf532d68d68))
- *(deps)* Bump shadow-rs from 0.36.1 to 0.37.0 (#49) - ([849fb1e](https://github.com/hadronomy/bootleg/commit/849fb1e3cf804a0b0cafd4e455bc32ee0337aa67))
- *(deps)* Bump thiserror from 2.0.6 to 2.0.7 (#48) - ([0faf8d7](https://github.com/hadronomy/bootleg/commit/0faf8d73a0e8e9e469a6d14abd48ddd0f4747eff))
- *(deps)* Bump shadow-rs from 0.36.0 to 0.36.1 (#47) - ([7b22fd0](https://github.com/hadronomy/bootleg/commit/7b22fd0e92cd6fd4a205c7a9bb6b82cc820fa198))
- *(deps)* Bump thiserror from 2.0.4 to 2.0.6 (#45) - ([d72cecc](https://github.com/hadronomy/bootleg/commit/d72cecc627af6bb3c4a31b78667ff0d7aaec203b))
- *(deps)* Bump const_format from 0.2.33 to 0.2.34 (#44) - ([6bedfab](https://github.com/hadronomy/bootleg/commit/6bedfab0a565ed93f07c9028415e5d435c5408fd))
- *(deps)* Bump clap from 4.5.22 to 4.5.23 (#43) - ([039371d](https://github.com/hadronomy/bootleg/commit/039371db6b0a48e00e02abe58911159ad1e768c8))
- *(deps)* Bump clap from 4.5.21 to 4.5.22 (#42) - ([6d60936](https://github.com/hadronomy/bootleg/commit/6d60936615fc1f4a56f0bc8cb847370e8cc76298))
- *(deps)* Bump thiserror from 2.0.3 to 2.0.4 (#41) - ([a62cc79](https://github.com/hadronomy/bootleg/commit/a62cc790d624b1351977266d2256075db4f2c122))
- *(deps)* Bump tracing-subscriber from 0.3.18 to 0.3.19 (#40) - ([36f94f6](https://github.com/hadronomy/bootleg/commit/36f94f6e35331932f9422b08fb0923a37278a1fc))
- *(deps)* Bump miette from 7.3.0 to 7.4.0 (#38) - ([e9fe3d0](https://github.com/hadronomy/bootleg/commit/e9fe3d0d243d5055a7f5ca46cf62f824abc95a3b))
- *(deps)* Bump tracing from 0.1.40 to 0.1.41 (#37) - ([8f648d3](https://github.com/hadronomy/bootleg/commit/8f648d393cddbaa3aead7b90fc9b43fc6910e7ad))
- *(deps)* Bump miette from 7.2.0 to 7.3.0 (#36) - ([f07f1cf](https://github.com/hadronomy/bootleg/commit/f07f1cfc1c51759688fe0f6f2d02763ac5c946a2))
- *(deps)* Bump termimad from 0.31.0 to 0.31.1 (#35) - ([739d3cb](https://github.com/hadronomy/bootleg/commit/739d3cb517be7cde73f85306330848a081035e25))
- *(deps)* Bump shadow-rs from 0.35.2 to 0.36.0 (#34) - ([31da449](https://github.com/hadronomy/bootleg/commit/31da449c03f3f466681029beeb819f9e11ba069f))
- *(deps)* Bump clap from 4.5.20 to 4.5.21 (#33) - ([3f51d48](https://github.com/hadronomy/bootleg/commit/3f51d48dcf15a861a8ad552300abe645eb726bfc))
- *(deps)* Bump thiserror from 2.0.1 to 2.0.3 (#32) - ([c638b0c](https://github.com/hadronomy/bootleg/commit/c638b0cc90dcf46ef72ef680a2fb3187959a1e73))
- *(deps)* Bump thiserror from 2.0.0 to 2.0.1 (#31) - ([2c50e8a](https://github.com/hadronomy/bootleg/commit/2c50e8a7a70e7b382eb4b3268263717447587b41))
- *(deps)* Bump thiserror from 1.0.68 to 2.0.0 (#30) - ([331d894](https://github.com/hadronomy/bootleg/commit/331d8948aed20415555ef4745fa144bbe2de1ccb))
- *(deps)* Bump thiserror from 1.0.66 to 1.0.68 (#28) - ([fe83526](https://github.com/hadronomy/bootleg/commit/fe83526df632c0a9560efb8d39a01668c0cf29ff))
- Enable github attestations for releases - ([20b3ae8](https://github.com/hadronomy/bootleg/commit/20b3ae82c905b2bb3019f459ba106fea55f6d5e0))
- Revert to `tar.gz` artifacts - ([e513a2a](https://github.com/hadronomy/bootleg/commit/e513a2a99685cb083c5dae3b7cefcc2fa2cc84fb))
- Add wayyy more commit parsers to changelog config - ([790511b](https://github.com/hadronomy/bootleg/commit/790511b389664192da87519e0fd3c90db20c194c))

### 📚 Documentation

- Fix bad link in `README` - ([194fa41](https://github.com/hadronomy/bootleg/commit/194fa410e3d1edaa86f6908087492b19a3fe89de))
- Add alternative installation methods to `README` - ([2de5891](https://github.com/hadronomy/bootleg/commit/2de5891da2aa66a4ef5d366888f559b68edfe048))

### 🎨 Styling

- Sort imports - ([ac985ad](https://github.com/hadronomy/bootleg/commit/ac985adef260703da0754c5676285e648f22ac22))
- Elide lifetimes - ([0b5a15b](https://github.com/hadronomy/bootleg/commit/0b5a15b7992f5db96f62f1635b60652f7f672aff))
- Fix formatting issues - ([c61dacc](https://github.com/hadronomy/bootleg/commit/c61dacce79e2b2da053ff92fab91272e143a2d2f))

### 🧪 Testing

- Add `print_help` test - ([f96a0fd](https://github.com/hadronomy/bootleg/commit/f96a0fd838ce405db343e62d41f73a8aed404bb3))

### ⚙️ Miscellaneous Tasks

- Set `clippy-all` as default bacon job - ([5248d6a](https://github.com/hadronomy/bootleg/commit/5248d6ab49c48de79ddfa36ea4185289ac6deb2e))
- Rename `.mise.toml` to `mise.toml` - ([e9f0518](https://github.com/hadronomy/bootleg/commit/e9f051866e09f51e57f52a2196650454b783ced6))
- Add `cargo-machete` to tools - ([e714034](https://github.com/hadronomy/bootleg/commit/e71403458c0b6281b2b682dcaed054fd188fb62a))
- Add `bacon` as default mise task - ([ec033d4](https://github.com/hadronomy/bootleg/commit/ec033d4a79644cbe0ab330a1a1adcc1d43366ebe))
- Add `.mise.toml` to manage tools and tasks - ([c7b81e3](https://github.com/hadronomy/bootleg/commit/c7b81e302b1ebfb4cbed58b01dcc5d4d4befc4a8))
- Fix `bacon` configuration - ([7a80ac3](https://github.com/hadronomy/bootleg/commit/7a80ac307c1fa1d04a7d5afafa5f33140cf2e2fa))
- Add `.vscode` to `.gitignore` - ([8a53212](https://github.com/hadronomy/bootleg/commit/8a5321273883fd7781db6313dcae5ef3ac46f2e4))
- Add `mutants.out` to `.gitignore` - ([19ef2e7](https://github.com/hadronomy/bootleg/commit/19ef2e79ee2f8150b6d9334582555d2d5db9ebf0))

## [0.1.12](https://github.com/hadronomy/bootleg/compare/v0.1.11...v0.1.12)

### 📚 Documentation

- Add execution example to `README` - ([b26b7fc](https://github.com/hadronomy/bootleg/commit/b26b7fc56dfc8f2956ced91adc26cd2242d556fc))

## [0.1.11](https://github.com/hadronomy/bootleg/compare/v0.1.10...v0.1.11)

### ⚙️ Miscellaneous Tasks

- Update `cargo-dist` version and builder images - ([654c292](https://github.com/hadronomy/bootleg/commit/654c2927200ba858f2c4c178cb39b1612eb253bd))

## [0.1.10](https://github.com/hadronomy/bootleg/compare/v0.1.9...v0.1.10)

### ⚙️ Miscellaneous Tasks

- Update mergify to ignore `hadronomy-bot` - ([48b299d](https://github.com/hadronomy/bootleg/commit/48b299d9096e79705eacfd22e8437909d98a92f7))
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
