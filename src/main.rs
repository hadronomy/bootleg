use arboard::Clipboard;
use miette::*;
use shadow_rs::shadow;
use std::io::{stdin, IsTerminal, Read};

use cli::*;

mod cli;
mod examples;

shadow!(build);

fn main() -> Result<()> {
    let cli = Args::init_cli();
    if stdin().is_terminal() {
        if cli.text.is_some() {
            let text = cli.text.unwrap();
            let mut clipboard = Clipboard::new().into_diagnostic()?;
            clipboard.set_text(&text).into_diagnostic()?;
        }
        return Ok(());
    }
    if cli.text.is_some() {
        return Err(miette!(
            labels = vec![LabeledSpan::new(
                Some("here".to_string()),
                build::PROJECT_NAME.len() + 1,
                cli.text.as_ref().unwrap().len()
            )],
            "Cannot read from stdin and provide text at the same time."
        )
        .with_source_code(format!("{} {}", build::PROJECT_NAME, cli.text.as_ref().unwrap())));
    }
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).into_diagnostic()?;
    let mut clipboard = Clipboard::new().into_diagnostic()?;
    clipboard.set_text(&buffer).into_diagnostic()?;
    Ok(())
}
