
use clap_complete::{generate_to, shells::Bash};
use std::env;

include!("src/arguments.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = _handle_command();
    generate_to(
        Bash,
        &mut cmd,
        "ddr-mount",
        outdir
    )?;

    Ok(())
}