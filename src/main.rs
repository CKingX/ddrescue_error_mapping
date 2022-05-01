mod error;
mod mount;
mod config;
mod unmount;

use clap::{Parser, Subcommand};
use std::{ffi::OsString};
use mount::*;
use config::list_devices;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mounts the image but with the map file to present I/O errors. 
    #[clap(long_about = "Mounts the image but with the map file to present I/O \
    errors. This is done by converting bad sectors and any areas not yet read \
    or skipped by ddrescue into I/O errors.")]
    Mount {
        #[clap(short,long)]
        /// Path to disk image
        image: OsString,
        #[clap(short,long)]
        /// Path to ddrescue map file
        map: OsString,
        /// Sector size of disk that was imaged
        #[clap(short,long,default_value_t = 512)]
        block_size: u32},
    /// Unmounts any image mounted by ddr_mount
    Unmount {
        /// Device previously mounted with ddrescue_error_mapping mount
        device: String
    },
    /// List mounted images and their mount points
    List,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Mount{image,map,block_size} => {
            error::check_root();
            mount(image,map,block_size);
        },
        Commands::Unmount { device } => {
            error::check_root();
            unmount::unmount(device)
        },
        Commands::List => {
            list_devices();
        },
    }
}