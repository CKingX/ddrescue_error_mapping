use std::{
    ffi::OsString,
    io::{ErrorKind, Write as IoWrite},
    process::{self, Command},
};

use crate::config::{self, DM_LOCATION};
use crate::error::{self, FileType};
use crate::parser::parse_map;
use std::io::Error as IOError;
use std::path::Path;

/// Mounts the image
pub fn mount(image: OsString, map: OsString, block_size: u32) {
    if block_size % 512 != 0 {
        error::sector_error();
    }

    let image = absolute_image_path(image);

    // mount the image
    let image_mount_path = losetup_mount(&image, block_size);
    let image_path = OsString::from(&image_mount_path);

    // mount the device mapper over image mount, creating error I/O range using map file
    let (entry, device_name) = dm_mount(&map, &image_path);

    let mut config = config::Config::read_config();
    config.write_device(image.clone(), entry, image_mount_path, device_name.clone());

    if let Some(x) = image.to_str() {
        println!("{x} is mounted at {DM_LOCATION}{device_name}");
    } else {
        println!("Image is mounted at {DM_LOCATION}{device_name}");
    }
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
        error::check_io_error(
            IOError::from(ErrorKind::NotFound),
            file_name,
            FileType::ImageFile,
        );
    }
}

/// Mounts the image using losetup. First, it finds an empty loop device using losetup -f
/// Then it mounts by doing:
/// ```losetup {loopdev} {path to image file} -b {sector size} -r```
/// Finally returns the loopdev path
fn losetup_mount(image: &OsString, block_size: u32) -> String {
    let losetup_next_loop_device = Command::new("losetup")
        .args(["-f"])
        .output()
        .unwrap_or_else(|_| error::mount_error());
    let image_mount_path = String::from_utf8(losetup_next_loop_device.stdout)
        .unwrap_or_else(|_| error::mount_error())
        .trim_matches('\n')
        .to_string();

    let image_mount_status = Command::new("losetup")
        .args([
            &OsString::from(&image_mount_path),
            image,
            &OsString::from("-r"),
            &OsString::from("-b"),
            &OsString::from(block_size.to_string()),
        ])
        .stdin(process::Stdio::null())
        .output()
        .unwrap_or_else(|_| error::mount_error());

    if !image_mount_status.status.success() {
        eprintln!("{}", String::from_utf8(image_mount_status.stderr).unwrap());
        error::mount_error();
    }

    image_mount_path
}

/// Mounts the image using the parse map with the following commmand
/// ```dmsetup create {device name}``` and passes parse map in stdin
fn dm_mount(map: &OsString, image_path: &OsString) -> (u32, String) {
    let entry = config::get_next_devices();
    let image_path = image_path.to_str().unwrap();

    let device_name = format!("{}{}", config::DEVICE_NAME, entry);
    let device_mapper = &parse_map(map, image_path);

    let mut dm_mount_process = Command::new("dmsetup")
        .args(["create", &device_name])
        .stdin(process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| error::mount_error_clean(image_path));
    dm_mount_process
        .stdin
        .take()
        .unwrap_or_else(|| error::mount_error_clean(image_path))
        .write_all(device_mapper.as_bytes())
        .unwrap_or_else(|_| error::mount_error_clean(image_path));

    dm_mount_process
        .wait_with_output()
        .unwrap_or_else(|_| error::mount_error_clean(image_path));

    (entry, device_name)
}
