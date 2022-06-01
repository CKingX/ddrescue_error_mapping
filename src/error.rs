use crate::unmount::*;
use colored::Colorize;
use log::error;
use std::fmt::Display;
use std::{env, io::ErrorKind, process};

/// Represents all the exit codes of the program with 0 being success and the rest being errors
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
    SectorSizeError = 9,
    UnknownError = 10,
}

pub enum FileType {
    MapFile,
    ImageFile,
}

pub enum Token {
    Pos,
    Size,
    CurrentPos,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Pos => "position".to_string(),
            Token::Size => "size".to_string(),
            Token::CurrentPos => "current position".to_string(),
        }
    }
}

impl FileType {
    pub fn to_string(&self) -> &str {
        match self {
            FileType::MapFile => "map file",
            FileType::ImageFile => "image file",
        }
    }
}

pub const SET_CONFIG_ERROR: &str = "Unable to open or set configuration";
pub const READ_CONFIG_ERROR: &str = "Unable to read configuration";
pub const OOM_ERROR: &str = "Out of memory error!";
pub const MOUNT_ERROR: &str = "Unable to mount image";
pub const NO_DEVICE_UNMOUNT_ERROR: &str = "Unmount error: Unable to find device";
pub const UNMOUNT_ERROR: &str = "Unable to unmount device";
pub const FILE_NOT_FOUND_ERROR: &str = "Unable to find";
pub const SECTOR_SIZE_ERROR: &str = "Sector size is not a multiple of 512";
pub const WSL_ERROR: &str = "ddr-mount does not support running under WSL";

// Parser errors
pub const CONTIGUOUS_ERROR: &str = "Position {pos} does not match size of previous line {size}\n\
Disk map is not contiguous or is overlapping";
pub const POSITION_SECTOR_ERROR: &str = "Position does not match sector size";
pub const SIZE_SECTOR_ERROR: &str = "Size does not match sector size";
pub const UNKNOWN_MAP_STATUS_ERROR: &str = "Unknown status character";
pub const NO_POSITION_ERROR: &str = "No position found";
pub const NO_SIZE_ERROR: &str = "No size found";
pub const NO_STATUS_ERROR: &str = "No status found";
pub const CONVERT_ERROR: &str = "Could not convert {entry} to decimal";
pub const START_NONZERO_ERROR: &str = "Disk map does not begin from 0";
pub const PARSE_ERROR: &str = "Unable to parse ddrescue map file";
pub const EMPTY_MAP_ERROR: &str = "Map file is empty";
pub const NO_CURRENT_POSITION_ERROR: &str = "Current position in status line is missing";
pub const NO_CURRENT_STATUS_ERROR: &str = "Current status is missing from the status line";
pub const UNKNOWN_CURRENT_STATUS_ERROR: &str = "Invalid status in status line";
pub const UNKNOWN_CURRRENT_PHASE_ERROR: &str = "Invalid phase in status line";
pub const CURRENT_PHASE_LESS_THAN_ONE_ERROR: &str =
    "Invalid phase in status line: phase must be 1 or greater";

pub fn file_not_found(filetype: FileType) -> String {
    format!("{FILE_NOT_FOUND_ERROR} {}", filetype.to_string())
}

pub fn wsl_error() -> ! {
    print_error(WSL_ERROR);
    std::process::exit(ExitCode::UnknownError as i32);
}

pub fn convert_error_string(token: Token) -> String {
    CONVERT_ERROR.replace("{entry}", &token.to_string())
}

pub fn parse_error(print: bool) -> ! {
    if print {
        print_error(PARSE_ERROR)
    };
    process::exit(ExitCode::ParseError as i32);
}

pub fn sector_error() -> ! {
    print_error(SECTOR_SIZE_ERROR);
    process::exit(ExitCode::SectorSizeError as i32);
}

pub fn print_error(error: impl Display) {
    let error_string = format!("{error}");
    eprintln!("{}", error_string.red().bold());
}

pub fn set_config_error() -> ! {
    print_error(SET_CONFIG_ERROR);
    process::exit(ExitCode::ConfigError as i32);
}

pub fn handle_string_write(result: Result<(), std::fmt::Error>) {
    if result.is_err() {
        print_error(OOM_ERROR);
        process::exit(ExitCode::OOMError as i32);
    }
}

pub fn read_config_error() -> ! {
    print_error(READ_CONFIG_ERROR);
    process::exit(ExitCode::ConfigError as i32);
}

pub fn root_error() {
    let env_vars = env::vars().find(|n| n.0 == "USER");
    if let Some((_, user)) = env_vars {
        error!("User running as {user}, rather than root");
        let arguments = env::args().reduce(|a, b| format!("{a} {b}")).unwrap();
        print_error(format!("You must run as root.\nTry sudo {arguments}"));
        process::exit(ExitCode::NonRoot as i32);
    }
}

pub fn check_io_error(error: std::io::Error, filename: String, filetype: FileType) -> ! {
    match error.kind() {
        ErrorKind::NotFound => {
            print_error(format!("{} {}", file_not_found(filetype), filename));
            process::exit(ExitCode::FileError as i32);
        }
        error => {
            print_error(format!("Unknown error while reading {filename} {}", error));
            process::exit(ExitCode::UnknownError as i32);
        }
    }
}

pub fn mount_error() -> ! {
    print_error(MOUNT_ERROR);
    process::exit(ExitCode::MountError as i32);
}

pub fn no_unmount_device(device: String) -> ! {
    print_error(format!("{NO_DEVICE_UNMOUNT_ERROR} {device}"));
    process::exit(ExitCode::UnmountError as i32);
}

pub fn unmount_error() -> ! {
    print_error(UNMOUNT_ERROR);
    process::exit(ExitCode::UnmountError as i32);
}

pub fn mount_error_clean(device: &str) -> ! {
    let _ = crate::unmount::unmount_image(device.to_string(), ImageError::HideError);
    mount_error();
}
