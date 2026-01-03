use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Clipboard operation failed")]
    #[diagnostic(
        code(bootleg::clipboard),
        help(
            "Could not interact with the system clipboard. Ensure your system supports it (e.g., xclip/xsel on Linux)."
        )
    )]
    ClipboardError {
        #[source]
        source: arboard::Error,
    },

    #[error("Input conflict detected")]
    #[diagnostic(
        code(bootleg::input_conflict),
        help("You provided text arguments AND piped input via stdin. Please choose one source.")
    )]
    StdinAndTextInputConflict,

    #[error("Failed to read input")]
    #[diagnostic(code(bootleg::io_read))]
    StdinReadError {
        #[source]
        source: std::io::Error,
    },

    #[error("Pager execution failed")]
    #[diagnostic(
        code(bootleg::pager),
        help("Failed to spawn the requested pager or viewer tool.")
    )]
    PagerError {
        #[source]
        source: std::io::Error,
    },
}
