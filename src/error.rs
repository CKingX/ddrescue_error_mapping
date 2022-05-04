use std::{env, io::ErrorKind};
use crate::unmount::*;

#[repr(i32)]
pub enum ExitCode {
    FileError = 1,
    _ArgumentError = 2,
    ConfigError = 3,
    MountError = 4,
    NonRoot = 5,
    OOMError = 6,
    ParseError = 7,
    UnmountError = 8,
    UnknownError = 10,
}

pub enum FileType {
    MapFile,
    ImageFile,
}

impl FileType {
    pub fn to_string(&self) -> &str {
        match self {
            FileType::MapFile => "map file",
            FileType::ImageFile => "image file",
        }
    }
}

pub const CONVERT_ERROR: &str = "Could not convert hex to decimal";
pub const PARSE_ERROR: &str = "Unable to parse ddrescue map file";
pub const SET_CONFIG_ERROR: &str = "Unable to open or set configuration";
pub const READ_CONFIG_ERROR: &str = "Unable to read configuration";
pub const OOM_ERROR: &str = "Out of memory error!";
pub const MOUNT_ERROR: &str = "Unable to mount image";
pub const NO_DEVICE_UNMOUNT_ERROR: &str = "Unmount error: Unable to find device";
pub const UNMOUNT_ERROR: &str = "Unable to unmount device";

pub fn file_not_found(filetype: FileType) -> String {
    format!("Unable to find {}", filetype.to_string())
}

pub fn set_config_error() -> ! {
    eprintln!("{}", SET_CONFIG_ERROR);
    std::process::exit(ExitCode::ConfigError as i32);
}

pub fn handle_string_write(result: Result<(), std::fmt::Error>) {
    if result.is_err() {
        eprintln!("{OOM_ERROR}");
        std::process::exit(ExitCode::OOMError as i32);
    }
}

pub fn read_config_error() -> ! {
    eprintln!("{}", READ_CONFIG_ERROR);
    std::process::exit(ExitCode::ConfigError as i32);
}

pub fn check_root() {
    let env_vars = env::vars().filter(|n| n.0 == "USER").next();
    if let Some((_, user)) = env_vars {
        if user != "root"{
            let arguments = env::args().reduce(|a,b| format!("{a} {b}")).unwrap(); 
            eprintln!("You must run as root.\nTry sudo {arguments}");
            std::process::exit(ExitCode::NonRoot as i32);
        }
    }
}

pub fn check_io_error(error: std::io::Error, filename: String, filetype: FileType) -> ! {
    match error.kind() {
        ErrorKind::NotFound => {
            eprintln!("{} {}", file_not_found(filetype), filename);
            std::process::exit(ExitCode::FileError as i32);
        }
        error => {
            eprintln!("Unknown error while reading {filename} {}", error.to_string());
            std::process::exit(ExitCode::UnknownError as i32);
        }
    }
}

pub fn mount_error() -> ! {
    eprintln!("{MOUNT_ERROR}");
    std::process::exit(ExitCode::MountError as i32);
}

pub fn no_unmount_device(device: String) -> ! {
    eprintln!("{NO_DEVICE_UNMOUNT_ERROR} {device}");
    std::process::exit(ExitCode::UnmountError as i32);
}

pub fn unmount_error() -> ! {
    eprintln!("{UNMOUNT_ERROR}");
    std::process::exit(ExitCode::UnmountError as i32);
}

pub fn mount_error_clean(device: &str) -> ! {
    let _ = crate::unmount::unmount_image(device.clone().to_string(), ImageError::HideError);
    mount_error();
}