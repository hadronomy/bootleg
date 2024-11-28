use arboard::Clipboard;
use miette::*;
use shadow_rs::shadow;
use std::io::{stdin, IsTerminal, Read};

use cli::*;

pub mod cli;
mod examples;

shadow!(build);

/// Runs the program.
pub fn run(cli: Args) -> Result<()> {
    if stdin().is_terminal() {
        handle_terminal_input(cli)
    } else {
        handle_non_terminal_input(cli)
    }
}

/// Handles terminal input by either setting the clipboard text or printing the clipboard text.
///
/// # Arguments
///
/// * `cli` - The command line arguments.
///
/// # Returns
///
/// * `Result<()>` - The result of the operation.
pub fn handle_terminal_input(cli: Args) -> Result<()> {
    if let Some(text) = cli.text {
        set_clipboard_text(&text)
    } else {
        print_clipboard_text()
    }
}

/// Handles non-terminal input by reading from stdin and setting the clipboard text.
///
/// # Arguments
///
/// * `cli` - The command line arguments.
///
/// # Returns
///
/// * `Result<()>` - The result of the operation.
///
/// # Errors
///
/// Returns an error if both stdin and text are provided.
pub fn handle_non_terminal_input(cli: Args) -> Result<()> {
    if cli.text.is_some() {
        return Err(miette!(
            labels = vec![LabeledSpan::new(
                Some("here".to_string()),
                build::PROJECT_NAME.len() + 1,
                cli.text.as_ref().unwrap().len()
            )],
            "Cannot read from stdin and provide text at the same time."
        )
        .with_source_code(format!(
            "{} {}",
            build::PROJECT_NAME,
            cli.text.as_ref().unwrap()
        )));
    }
    read_from_stdin_and_set_clipboard(stdin())
}

/// Sets the clipboard text.
///
/// # Arguments
///
/// * `text` - The text to set in the clipboard.
///
/// # Returns
///
/// * `Result<()>` - The result of the operation.
///
/// # Errors
///
/// Returns an error if setting the clipboard text fails.
pub fn set_clipboard_text(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().into_diagnostic()?;
    clipboard.set_text(text).into_diagnostic()?;
    Ok(())
}

/// Prints the clipboard text.
///
/// # Returns
///
/// * `Result<()>` - The result of the operation.
///
/// # Errors
///
/// Returns an error if getting the clipboard text fails.
pub fn print_clipboard_text() -> Result<()> {
    let mut clipboard = Clipboard::new().into_diagnostic()?;
    let text = clipboard.get_text().into_diagnostic()?;
    print!("{}", text);
    Ok(())
}

/// Reads from a reader and sets the clipboard text.
///
/// # Arguments
///
/// * `reader` - The reader to read from.
///
/// # Returns
///
/// * `Result<()>` - The result of the operation.
///
/// # Errors
///
/// Returns an error if reading from the reader
/// or setting the clipboard text fails.
pub fn read_from_stdin_and_set_clipboard<R: Read>(mut reader: R) -> Result<()> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).into_diagnostic()?;
    set_clipboard_text(&buffer)
}
