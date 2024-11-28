use miette::*;

use bootleg::{cli::*, *};

fn main() -> Result<()> {
    let cli = Args::init_cli();
    run(cli)
}
