use clap_complete::{generate_to, shells::Bash};
use std::env;
use clap_mangen::Man;

include!("src/arguments.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = _handle_command();
    generate_to(Bash, &mut cmd, "ddr-mount", outdir.clone())?;
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(std::path::PathBuf::from(outdir).join("ddr-mount.1"), buffer)?;

    Ok(())
}
