mod arguments;
mod config;
mod error;
mod mount;
mod parser;
mod unmount;

use arguments::*;
use config::list_devices;
use mount::*;

fn main() {
    let args = handle_arguments();

    match args {
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
