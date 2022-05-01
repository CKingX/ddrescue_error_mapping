use std::{ffi::{OsString}, fs, fmt::Write, process::{self, Command}, io::ErrorKind};

use crate::error::{self, FileType};
use crate::config::{self, DM_LOCATION};
use std::path::Path;
use std::io::Error as IOError;

pub fn mount(image: OsString, map: OsString, block_size: u32) {

    let image = absolute_image_path(image);
    
    // mount the image
    let image_mount_path = losetup_mount(&image, block_size);
    let image_path = OsString::from(&image_mount_path);

    // mount the device mapper over image mount, creating error I/O range using map file
    let entry = config::get_next_devices();

    let device_name = format!("{}{}",config::DEVICE_NAME,entry);
    let device_mapper = &format!("{}",parse_map(map, &image_path.to_str().unwrap()));

    let dm_mount_status = Command::new("dmsetup")
                            .args(["create",&device_name,"--table",&device_mapper])
                            .stdin(process::Stdio::null())
                            .output().unwrap_or_else(|_| error::mount_error());

    if !dm_mount_status.status.success() {
        eprintln!("Unable to read image file");
        std::process::exit(error::ExitCode::FileError as i32);
    }
    
    let mut config = config::Config::read_config();
    config.write_device(image.clone(), entry, image_mount_path, device_name.clone());

    if let Some(x) = image.to_str() {
        println!("{x} is mounted at {DM_LOCATION}{device_name}");
    } else {
        println!("Image is mounted at {DM_LOCATION}{device_name}");
    }
}

fn parse_map(map_path: OsString, device_name: &str) -> String {
    let mut output = String::new();
    let contents= fs::read_to_string(map_path.clone())
                    .unwrap_or_else(|error| error::check_io_error(error, 
                        map_path.into_string().unwrap_or_default(),
                        crate::error::FileType::MapFile));

    let mut file_line = contents.lines()
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty() && !s.contains("#"))
                        .skip(1);

    while let Some(line) = file_line.next() {
        let pos_string: String;
        let pos: u64;
        let size: u64;
        let size_string: String;
        let status: char;

        let mut map_line = line.split_ascii_whitespace();
        pos_string = map_line.next().expect(error::PARSE_ERROR).to_string();
        size_string = map_line.next().expect(error::PARSE_ERROR).to_string();
        status = map_line.next().expect(error::PARSE_ERROR).chars().nth(0)
                    .unwrap();

        pos = u64::from_str_radix(pos_string.trim_start_matches("0x"),16)
                .expect(error::CONVERT_ERROR);
        size = u64::from_str_radix(size_string.trim_start_matches("0x"),16)
                .expect(error::CONVERT_ERROR);
        
        if status == '+' {
            error::handle_string_write(writeln!(output,"{}", 
                        create_linear(pos,size, &device_name)));
        } else {
            error::handle_string_write(writeln!(output, "{}",
                        create_error(pos, size)));
        }
    }

    output
}

/// Creates dmtable for error device
fn create_error(pos: u64, size: u64) -> String {
    format!("{} {} error", pos/512, size/512)
}

/// Creates dmtable for linear device
fn create_linear(pos: u64, size: u64, device: &str) -> String {
    format!("{} {} linear {device} {}", pos/512, size/512, pos/512)
}

/// Gets the absolute path of the image
fn absolute_image_path(path: OsString) -> OsString {
    let path: &Path = path.as_ref();
        let result = path.canonicalize();
        if let Ok(x) = result {
            x.as_os_str().to_os_string()
        } else {
            let file_name = if let Some(x) = path.to_str() {
                x.to_string()
            } else {
                "".to_string()
            };
            error::check_io_error(IOError::from(ErrorKind::NotFound),
            file_name,FileType::ImageFile);
        }
}

/// Mounts the image using losetup. First, it finds an empty loop device using losetup -f
/// Then it mounts by doing losetup {loopdev} {path to image file} -b {sector size}
/// Finally returns the loopdev path
fn losetup_mount(image: &OsString, block_size: u32) -> String {
    let losetup_next_loop_device = Command::new("losetup").args(["-f"])
                            .output().unwrap_or_else(|_| error::mount_error());
    let image_mount_path = String::from_utf8(losetup_next_loop_device.stdout)
        .unwrap_or_else(|_| error::mount_error())
        .trim_matches('\n').to_string();

    let image_mount_status = Command::new("losetup")
                            .args([&OsString::from(&image_mount_path), 
                            &image, &OsString::from("-r"),
                            &OsString::from("-b"),
                            &OsString::from(block_size.to_string())])
                            .stdin(process::Stdio::null())
                            .output().unwrap_or_else(|_| error::mount_error());

    if !image_mount_status.status.success() {
        eprintln!("{}", String::from_utf8(image_mount_status.stderr).unwrap());
        error::mount_error();
    }

    image_mount_path
}

fn dm_mount() -> String {
    todo!();
}