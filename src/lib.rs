use arboard::Clipboard;
use miette::*;
use shadow_rs::shadow;
use std::io::{IsTerminal, Read, stdin};

use cli::*;
use error::Error;

pub mod cli;
pub mod error;
mod examples;
mod viewer;

shadow!(build);

/// Runs the program with automatic input detection.
pub fn run(cli: Args) -> Result<()> {
    // Case 1: Arguments provided via CLI (Highest Priority)
    if let Some(text) = &cli.text {
        // Validation: Cannot pipe AND provide args
        if !stdin().is_terminal() {
            return Err(Error::StdinAndTextInputConflict).into_diagnostic();
        }
        return set_clipboard_text(text);
    }

    // Case 2: Piped input provided (STDIN is not a TTY)
    if !stdin().is_terminal() {
        return read_from_stdin_and_set_clipboard(stdin());
    }

    // Case 3: No args, No pipe -> View/Paste mode
    print_clipboard_text(cli)
}

fn set_clipboard_text(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::ClipboardError { source: e })
        .into_diagnostic()?;

    clipboard
        .set_text(text)
        .map_err(|e| Error::ClipboardError { source: e })
        .into_diagnostic()?;

    Ok(())
}

fn read_from_stdin_and_set_clipboard<R: Read>(mut reader: R) -> Result<()> {
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .map_err(|e| Error::StdinReadError { source: e })
        .into_diagnostic()?;

    let text = buffer.trim_end();
    set_clipboard_text(text)
}

fn print_clipboard_text(cli: Args) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::ClipboardError { source: e })
        .into_diagnostic()?;

    let text = clipboard
        .get_text()
        .map_err(|e| Error::ClipboardError { source: e })
        .into_diagnostic()?;

    // Hand off to the viewer module to decide how to render (bat, fzf, plain, etc)
    viewer::display_content(&text, cli.pager, cli.lang, cli.plain).into_diagnostic()?;

    Ok(())
}
