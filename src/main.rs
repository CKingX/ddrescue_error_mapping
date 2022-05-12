mod arguments;
mod config;
mod error;
mod mount;
mod parser;
mod unmount;

use arguments::*;
use config::list_devices;
use mount::*;

use log::info;

fn main() {
    let args = handle_arguments();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("ddr-mount {}", env!("CARGO_PKG_VERSION"));

    match args.command {
        Commands::Mount {
            image,
            map,
            block_size,
        } => {
            error::check_root();
            mount(image, map, block_size);
        }
        Commands::Unmount { device } => {
            error::check_root();
            unmount::unmount(device)
        }
        Commands::UnmountAll => {
            error::check_root();
            unmount::unmount_all();
        }
        Commands::List => {
            list_devices();
        }
    }
}
