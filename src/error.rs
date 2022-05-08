use crate::unmount::*;
use atty::{self, Stream};
use std::fmt::Display;
use std::io::Write;
use std::{env, io::ErrorKind, process};
use termcolor::{self, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    /// Sector Size not a multiple of 512
    SectorSizeError = 9,
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
pub const FILE_NOT_FOUND_ERROR: &str = "Unable to find";
pub const SECTOR_SIZE_ERROR: &str = "Sector size is not a multiple of 512";

pub fn file_not_found(filetype: FileType) -> String {
    format!("{FILE_NOT_FOUND_ERROR} {}", filetype.to_string())
}

pub fn sector_error() -> ! {
    print_error(SECTOR_SIZE_ERROR);
    process::exit(ExitCode::SectorSizeError as i32);
}

fn print_error(error: impl Display) {
    if atty::is(Stream::Stderr) {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
        let _ = writeln!(&mut stderr, "{}", error);
    } else {
        eprintln!("{}", error);
    }
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

pub fn check_root() {
    let env_vars = env::vars().find(|n| n.0 == "USER");
    if let Some((_, user)) = env_vars {
        if user != "root" {
            let arguments = env::args().reduce(|a, b| format!("{a} {b}")).unwrap();
            print_error(format!("You must run as root.\nTry sudo {arguments}"));
            process::exit(ExitCode::NonRoot as i32);
        }
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
