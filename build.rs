use clap_complete::{generate_to, shells::Bash, shells::Fish};
use clap_mangen::Man;
use std::env;

include!("src/arguments.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let name = "ddr-mount";

    let mut cmd = _handle_command();
    #[allow(deprecated)]
    cmd._build_all();
    generate_to(Bash, &mut cmd, name, outdir.clone())?;
    generate_to(Fish, &mut cmd, name, outdir.clone())?;
    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(
        std::path::PathBuf::from(&outdir).join("ddr-mount.1"),
        buffer,
    )?;

    for subcommand in cmd.get_subcommands() {
        let subcommand_name = subcommand.get_name();
        let subcommand_name = format!("{name}-{subcommand_name}");
        let mut buffer: Vec<u8> = Default::default();
        let man = Man::new(subcommand.clone().name(&subcommand_name));
        man.render(&mut buffer)?;
        std::fs::write(
            std::path::PathBuf::from(&outdir).join(format!("{}{}", &subcommand_name, ".1")),
            buffer,
        )?;
    }

    Ok(())
}
