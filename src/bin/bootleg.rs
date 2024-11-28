use human_panic::setup_panic;
use miette::*;

use bootleg::{cli::*, *};

fn main() -> Result<()> {
    setup_panic!();
    let cli = Args::init_cli();
    run(cli)
}
