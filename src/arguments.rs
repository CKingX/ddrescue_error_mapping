use clap::{IntoApp, Parser, Subcommand};
use std::ffi::OsString;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mounts the image but with the map file to present I/O errors.
    #[clap(long_about = "Mounts the image but with the map file to present I/O \
    errors. This is done by converting bad sectors and any areas not yet read \
    or skipped by ddrescue into I/O errors.")]
    Mount {
        #[clap(short, long)]
        /// Path to disk image
        image: OsString,
        #[clap(short, long)]
        /// Path to ddrescue map file
        map: OsString,
        /// Sector size of disk that was imaged
        #[clap(short, long, default_value_t = 512)]
        block_size: u32,
    },
    /// Unmounts any image mounted by ddr-mount
    Unmount {
        /// Device previously mounted with ddr-mount mount (ex: ddrm0)
        device: String,
    },
    /// Unmounts all images mounted by ddr-mount
    UnmountAll,
    /// List mounted images and their mount points
    List,
}

pub fn handle_arguments() -> Commands {
    Cli::parse().command
}

#[allow(deprecated)]
pub fn _handle_command() -> clap::App<'static> {
    Cli::command()
}
