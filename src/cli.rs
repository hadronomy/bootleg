use std::fmt;
use std::process;

use clap::{CommandFactory, Parser};
use clap_help::Printer;
use owo_colors::{OwoColorize, Style};
use serde::Serialize;
use shadow_rs::formatcp;
use supports_color::Stream;
use taplo::formatter::Options;
use taplo::rowan::{NodeOrToken, WalkEvent};
use termimad::ansi;

use crate::build;
use crate::examples::*;

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

    /// Use a specific pager (defaults to internal bat, then less, or env $PAGER)
    /// Special values: "bat" (internal), "fzf" (external).
    #[arg(short = 'p', long)]
    pub pager: Option<String>,

    /// Specify the language for syntax highlighting (only used if showing output)
    #[arg(short = 'l', long)]
    pub lang: Option<String>,

    /// Force plain text output (disable paging and highlighting)
    #[arg(long, conflicts_with = "pager")]
    pub plain: bool,
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
        skin.italic.set_fg(color);
        printer.print_help();
    }

    /// Prints the version information for the CLI.
    pub fn print_version() {
        let mut version_info = VersionInfoDisplay::new();
        let color = Color::Auto;
        if color.supports_color_on(Stream::Stdout) {
            version_info.colorize();
        }
        if build::BUILD_RUST_CHANNEL == "debug" {
            let text = " DEBUG BUILD ";
            if supports_color::on_cached(Stream::Stdout).is_some() {
                eprintln!("{}", text.on_yellow().black().bold());
            } else {
                eprintln!("{}", text);
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

#[derive(Debug, Serialize)]
struct VersionInfoDisplay<'a> {
    #[serde(rename = "version")]
    pkg_version: &'a str,
    branch: &'a str,
    commit_hash: &'a str,
    build_time: &'a str,
    build_env: &'a str,
    build_channel: &'a str,
    #[serde(skip)]
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
        let toml = toml::to_string(&self).map_err(|_| fmt::Error)?;

        if self.styles.name_style == Style::new() && self.styles.value_style == Style::new() {
            return write!(f, "{}", toml);
        }

        let formatted = taplo::formatter::format(
            &toml,
            Options {
                align_entries: true,
                ..Default::default()
            },
        );
        let syntax = taplo::parser::parse(&formatted);
        if !syntax.errors.is_empty() {
            return Err(fmt::Error);
        }

        let mut output = String::with_capacity(formatted.len());

        let tokens = syntax
            .into_syntax()
            .preorder_with_tokens()
            .filter_map(|event| match event {
                WalkEvent::Enter(NodeOrToken::Token(token)) => Some(token),
                _ => None,
            });

        for token in tokens {
            let text = token.text();
            match token.kind() {
                taplo::syntax::SyntaxKind::STRING
                | taplo::syntax::SyntaxKind::INTEGER
                | taplo::syntax::SyntaxKind::FLOAT => {
                    output.push_str(&text.style(self.styles.value_style).to_string());
                }
                taplo::syntax::SyntaxKind::IDENT | taplo::syntax::SyntaxKind::PERIOD => {
                    output.push_str(&text.style(self.styles.name_style).to_string());
                }
                _ => output.push_str(text),
            }
        }

        write!(f, "{}", output)
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
