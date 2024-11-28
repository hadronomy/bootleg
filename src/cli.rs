use std::process;

use crate::build;
use crate::examples::*;

use clap::{CommandFactory, Parser};
use clap_help::Printer;
use owo_colors::{OwoColorize, Style};
use shadow_rs::formatcp;
use std::fmt;
use supports_color::Stream;
use termimad::ansi;

static INTRO: &str = formatcp!(
    r#"
*{}* is a cli tool for copying to the clipboard

> **NOTE**: This is a work in progress and is missing some features.

"#,
    build::PROJECT_NAME
);

static EXAMPLES_TEMPLATE: &str = "
**Examples:**

${examples
**${example-number})** ${example-title}: 

```sh
${example-cmd}
```

> *${example-comments}*
---
}
";

#[derive(Parser, Debug)]
#[command(name = build::PROJECT_NAME, author, about, disable_help_flag = true)]
pub struct Args {
    /// Print help
    #[arg(short, long)]
    pub help: bool,

    /// Print version
    #[arg(short = 'V', long)]
    pub version: bool,

    /// The text to copy to the clipboard
    #[arg(name = "TEXT")]
    pub text: Option<String>,
}

/// Implements the `Args` struct and its associated methods.
impl Args {
    /// Initializes the command-line interface (CLI) and returns an `Args` object.
    /// ```
    /// let args = Args::init_cli();
    /// ```
    /// `exit` on error
    ///
    /// # Returns
    ///
    /// An `Args` object with the parsed command-line arguments.
    ///
    pub fn init_cli() -> Args {
        let args = Self::parse();
        if args.help {
            Self::print_help();
            process::exit(0)
        }
        if args.version {
            Self::print_version();
            process::exit(0)
        }
        args
    }

    /// Prints the help message for the CLI.
    ///
    /// The help message is styled using the `clap-help` and
    /// the `termimad` crates.
    pub fn print_help() {
        let mut printer = Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        printer.template_keys_mut().push("examples");
        printer.set_template("examples", EXAMPLES_TEMPLATE);
        for (i, example) in EXAMPLES.iter().enumerate() {
            printer
                .expander_mut()
                .sub("examples")
                .set("example-number", i + 1)
                .set("example-title", example.title)
                .set("example-cmd", example.cmd)
                .set_md("example-comments", example.comments);
        }
        let skin = printer.skin_mut();
        let color = ansi(190);
        skin.headers[0].compound_style.set_fg(color);
        skin.bold.set_fg(color);
        skin.italic = termimad::CompoundStyle::with_fg(color);
        printer.print_help();
    }

    /// Prints the version information for the CLI.
    pub fn print_version() {
        let mut version_info = VersionInfoDisplay::new();
        let color = Color::Auto;
        // Example usage of Color enum
        if color.supports_color_on(Stream::Stdout) {
            version_info.colorize();
        }
        if build::BUILD_RUST_CHANNEL == "debug" {
            let text = " DEBUG BUILD ";
            if supports_color::on_cached(Stream::Stdout).is_some() {
                println!("{}", text.on_yellow().black().bold());
            } else {
                println!("{}", text);
            }
        }
        println!("{}", version_info);
    }
}

#[derive(Debug, Default)]
struct Styles {
    name_style: Style,
    value_style: Style,
}

impl Styles {
    fn colorize(&mut self) {
        self.name_style = Style::new().blue();
        self.value_style = Style::new().yellow();
    }
}

struct VersionInfoDisplay<'a> {
    pkg_version: &'a str,
    branch: &'a str,
    commit_hash: &'a str,
    build_time: &'a str,
    build_env: &'a str,
    build_channel: &'a str,
    styles: Box<Styles>,
}

impl VersionInfoDisplay<'_> {
    fn new() -> Self {
        #[allow(clippy::const_is_empty)]
        let pkg_version = if build::TAG.is_empty() {
            formatcp!("{}-dev", build::PKG_VERSION)
        } else {
            build::PKG_VERSION
        };

        #[allow(clippy::const_is_empty)]
        let commit_hash = if !build::GIT_CLEAN {
            formatcp!("{}+", build::SHORT_COMMIT)
        } else {
            build::SHORT_COMMIT
        };

        Self {
            pkg_version,
            branch: build::BRANCH,
            commit_hash,
            build_time: build::BUILD_TIME,
            build_env: build::RUST_VERSION,
            build_channel: build::RUST_CHANNEL,
            styles: Box::default(),
        }
    }

    fn colorize(&mut self) {
        self.styles.colorize();
    }
}

impl fmt::Display for VersionInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<13}: {}\n{:<13}: {}\n{:<13}: {}\n{:<13}: {}\n{:<13}: {}\n{:<12}: {}",
            "pkg_version".style(self.styles.name_style),
            self.pkg_version.style(self.styles.value_style),
            "branch".style(self.styles.name_style),
            self.branch.style(self.styles.value_style),
            "commit_hash".style(self.styles.name_style),
            self.commit_hash.style(self.styles.value_style),
            "build_time".style(self.styles.name_style),
            self.build_time.style(self.styles.value_style),
            "build_env".style(self.styles.name_style),
            self.build_env.style(self.styles.value_style),
            "build_channel".style(self.styles.name_style),
            self.build_channel.style(self.styles.value_style)
        )
    }
}

#[derive(Debug)]
pub enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn supports_color_on(self, stream: Stream) -> bool {
        match self {
            Color::Always => true,
            Color::Auto => supports_color::on_cached(stream).is_some(),
            Color::Never => false,
        }
    }
}
