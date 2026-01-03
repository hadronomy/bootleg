use crate::error::Error;
use bat::PrettyPrinter;
use std::io::Write;
use std::process::{Command, Stdio};

/// Decides how to display the content:
/// 1. If `--plain` is set or stdout is not a TTY -> Print raw.
/// 2. If user requested specific pager -> Try to use it.
/// 3. Default -> Use embedded `bat` with auto-detected syntax.
pub fn display_content(
    content: &str,
    pager_cmd: Option<String>,
    language: Option<String>,
    plain: bool,
) -> Result<(), Error> {
    // If we are piping output (e.g. `bootleg | grep foo` or `bootleg | echo`),
    // we must handle BrokenPipe gracefully.
    if plain || !std::io::IsTerminal::is_terminal(&std::io::stdout()) {
        // Use write_all instead of print! to avoid panicking on BrokenPipe
        if let Err(e) = std::io::stdout().write_all(content.as_bytes()) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                // Pipe was closed (e.g. by `head` or `echo`), exit silently.
                return Ok(());
            }
            return Err(Error::PagerError { source: e });
        }
        return Ok(());
    }

    // Determine which pager to use
    match pager_cmd.as_deref() {
        // User explicitly asked for internal bat
        Some("bat") => run_internal_bat(content, language, None),

        // User asked for specific external tool (fzf, less, etc.)
        Some(cmd) => run_external_pager(content, cmd),

        // No preference -> Default to internal bat (which handles paging internally)
        None => run_internal_bat(content, language, None),
    }
}

/// Uses the embedded `bat` library to print with syntax highlighting and paging.
fn run_internal_bat(
    content: &str,
    language: Option<String>,
    _external_pager: Option<&str>,
) -> Result<(), Error> {
    let mut printer = PrettyPrinter::new();

    printer
        .input_from_bytes(content.as_bytes())
        .grid(true)
        .line_numbers(true)
        .paging_mode(bat::PagingMode::QuitIfOneScreen);

    // 1. Prefer user explicitly provided language
    // 2. Fallback to auto-detection
    // Note: We detect syntax based on a sample to avoid scanning massive inputs
    let detected_lang = detect_syntax(content);
    let lang_to_use = language.as_deref().or(detected_lang);

    if let Some(lang) = lang_to_use {
        printer.language(lang);
    }

    // Attempt to run bat. If it fails, fallback to raw print.
    printer.print().map_err(|e| Error::PagerError {
        source: std::io::Error::other(format!("Bat failed: {}", e)),
    })?;

    Ok(())
}

/// Spawns an external process (like `fzf`, `less`, `more`) and pipes content to it.
fn run_external_pager(content: &str, cmd_str: &str) -> Result<(), Error> {
    // Parse command string (e.g., "fzf --reverse --preview '...'")
    let args = shell_words::split(cmd_str).map_err(|e| Error::PagerError {
        source: std::io::Error::new(std::io::ErrorKind::InvalidInput, e),
    })?;

    if args.is_empty() {
        return Err(Error::PagerError {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "Empty pager command"),
        });
    }

    let program = &args[0];
    let program_args = &args[1..];

    // Check availability of the tool
    if which::which(program).is_err() {
        eprintln!(
            "Warning: '{}' not found in PATH. Falling back to internal viewer.",
            program
        );
        return run_internal_bat(content, None, None);
    }

    let mut child = Command::new(program)
        .args(program_args)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| Error::PagerError { source: e })?;

    if let Some(mut stdin) = child.stdin.take()
        && let Err(e) = stdin.write_all(content.as_bytes())
    {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            return Ok(());
        }
        return Err(Error::PagerError { source: e });
    }

    child.wait().map_err(|e| Error::PagerError { source: e })?;

    Ok(())
}

/// Heuristic syntax detection.
/// Returns the `bat` language identifier (e.g., "rs", "json", "py").
fn detect_syntax(content: &str) -> Option<&'static str> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Fast byte comparison for start/end characters.
    let bytes = trimmed.as_bytes();
    match (bytes.first(), bytes.last()) {
        (Some(b'{'), Some(b'}')) | (Some(b'['), Some(b']')) => return Some("json"),
        (Some(b'<'), Some(b'>')) => {
            if trimmed.starts_with("<?xml")
                || trimmed.starts_with("<!DOCTYPE")
                || trimmed.starts_with("<html")
            {
                return Some("html");
            }
        }
        _ => {}
    }

    // Shebang Analysis
    if trimmed.starts_with("#!") {
        // Safe to unwrap_or empty because string is known to start with characters
        let line = trimmed.lines().next().unwrap_or("");

        // Case-insensitive check for common interpreters
        let line_lower = line.to_lowercase();
        if line_lower.contains("python") {
            return Some("python");
        }
        if line_lower.contains("node") || line_lower.contains("deno") {
            return Some("javascript");
        }
        if line_lower.contains("bash") || line_lower.contains("sh") {
            return Some("bash");
        }
        if line_lower.contains("ruby") {
            return Some("ruby");
        }
        if line_lower.contains("perl") {
            return Some("perl");
        }
        if line_lower.contains("php") {
            return Some("php");
        }
        if line_lower.contains("rs") || line_lower.contains("rust") {
            return Some("rust");
        }
        if line_lower.contains("lua") {
            return Some("lua");
        }
        if line_lower.contains("go") {
            return Some("go");
        }
    }

    // Only scan the first 1KB to avoid performance hits on large clipboard contents.
    const SCAN_LIMIT: usize = 1024;
    let snippet = &trimmed[..trimmed.len().min(SCAN_LIMIT)];

    // Rust
    if snippet.contains("fn main")
        || snippet.contains("use std::")
        || snippet.contains("#[derive(")
        || snippet.contains("pub mod ")
    {
        return Some("rust");
    }

    // C / C++
    if snippet.starts_with("#include") || snippet.contains("int main(") {
        return Some("cpp");
    }

    // Go
    if snippet.starts_with("package ")
        && (snippet.contains("func ") || snippet.contains("import ("))
    {
        return Some("go");
    }

    // Java
    if snippet.contains("public static void main")
        || (snippet.contains("public class") && snippet.contains("package "))
    {
        return Some("java");
    }

    // Python (Fallback if no shebang)
    if (snippet.contains("def ") || snippet.contains("class "))
        && snippet.contains("import ")
        && snippet.contains(":")
    {
        return Some("python");
    }

    // SQL
    if snippet.starts_with("SELECT ")
        || snippet.starts_with("INSERT INTO ")
        || snippet.starts_with("CREATE TABLE")
    {
        return Some("sql");
    }

    None
}
