use miette::*;
use std::io::{stdin, IsTerminal};

use bootleg::{cli::*, *};

fn main() -> Result<()> {
    let cli = Args::init_cli();
    if stdin().is_terminal() {
        handle_terminal_input(cli)
    } else {
        handle_non_terminal_input(cli)
    }
}

