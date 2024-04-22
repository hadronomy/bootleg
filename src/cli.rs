use crate::examples::*;

use clap::{CommandFactory, Parser};
use clap_help::Printer;
use termimad::ansi;

static INTRO: &str = r#"
*bootleg* is a cli tool for copying to the clipboard

> **NOTE**: This is a work in progress and is missing some features.

"#;

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
#[command(name = "bootleg", author, version, about, disable_help_flag = true)]
pub struct Args {
    /// Print help
    #[arg(short, long)]
    pub help: bool,
    
    /// The text to copy to the clipboard
    #[arg(name = "TEXT")]
    pub text: Option<String>,
}

/// Implements the `Args` struct and its associated methods.
impl Args {
    /// Initializes the command-line interface (CLI) and returns an `Option<Args>` object.
    /// ```
    /// let args = Args::init_cli();
    /// ```
    ///
    /// # Returns
    ///
    /// - `Some(Args)`: If the CLI arguments were successfully parsed.
    /// - `None`: If the `help` flag is set, the help message is printed and `None` is returned.
    pub fn init_cli() -> Self {
        let args = Self::parse();
        if args.help {
            Self::print_help();
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
}
