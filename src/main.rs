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
use sudo::escalate_if_needed;
use update_informer::{registry, Check};

struct UnknownWSLStatus;

enum WSLStatus {
    RunningUnderWSL,
    RunningUnderLinux,
}

impl From<std::io::Error> for UnknownWSLStatus {
    fn from(_: std::io::Error) -> Self {
        UnknownWSLStatus
    }
}

impl From<std::string::FromUtf8Error> for UnknownWSLStatus {
    fn from(_: std::string::FromUtf8Error) -> Self {
        UnknownWSLStatus
    }
}

fn main() {
    let args = handle_arguments();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    info!("ddr-mount v{}", version);

    let informer = update_informer::new(registry::Crates, name, version);
    if let Ok(Some(version)) = informer.check_version() {
        println!("New version is available: {version}");
    }

    match wsl_check() {
        Ok(WSLStatus::RunningUnderWSL) => error::wsl_error(),
        Ok(_) => info!("Running under Linux"),
        Err(_) => info!("Unknown if running under WSL"),
    }

    match args.command {
        Commands::Mount {
            image,
            map,
            block_size,
        } => {
            ensure_root();
            mount(image, map, block_size);
        }
        Commands::Unmount { device } => {
            ensure_root();
            unmount::unmount(device)
        }
        Commands::UnmountAll => {
            ensure_root();
            unmount::unmount_all();
        }
        Commands::List => {
            list_devices();
        }
    }
}

fn ensure_root() {
    match escalate_if_needed() {
        Ok(_) => (),
        Err(_) => error::root_error(),
    };
}

fn wsl_check() -> Result<WSLStatus, UnknownWSLStatus> {
    let output = std::process::Command::new("uname").arg("-r").output()?;

    if output.status.success() {
        let mut wsl_status = String::from_utf8(output.stdout)?;
        wsl_status.make_ascii_lowercase();
        if wsl_status.contains("microsoft") || wsl_status.contains("wsl") {
            Ok(WSLStatus::RunningUnderWSL)
        } else {
            Ok(WSLStatus::RunningUnderLinux)
        }
    } else {
        Err(UnknownWSLStatus)
    }
}
