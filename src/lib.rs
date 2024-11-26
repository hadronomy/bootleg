use arboard::Clipboard;
use miette::*;
use shadow_rs::shadow;
use std::io::{stdin, Read};

use cli::*;

pub mod cli;
mod examples;

shadow!(build);

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
    read_from_stdin_and_set_clipboard()
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

/// Reads from stdin and sets the clipboard text.
/// 
/// # Returns
/// 
/// * `Result<()>` - The result of the operation.
/// 
/// # Errors
/// 
/// Returns an error if reading from stdin
/// or setting the clipboard text fails.
pub fn read_from_stdin_and_set_clipboard() -> Result<()> {
  let mut buffer = String::new();
  stdin().read_to_string(&mut buffer).into_diagnostic()?;
  let mut clipboard = Clipboard::new().into_diagnostic()?;
  clipboard.set_text(&buffer).into_diagnostic()?;
  Ok(())
}