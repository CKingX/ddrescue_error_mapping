use std::process::Command;

use crate::config::{self, Config, Device};
use crate::error;
use log::{error, info};

pub enum ImageError {
    ShowError,
    HideError,
}

/// Unmounts a device
pub fn unmount(device_name: String) {
    let mut config = Config::read_config();
    info!("Unmounting device {device_name}");
    let mut devices = config.iter_mut();
    let search = devices.find(|d| d.device_mount_point == device_name);

    if let Some(device) = search {
        unmount_device(device, Some(&mut config));
        info!("Successfully unmounted device");
    } else {
        error!("Could not find device");
        error::no_unmount_device(device_name);
    }
}

/// Unmounts device from device mapper
pub fn unmount_device_mapper(name: &str) {
    info!("dmsetup remove {name}");
    let output = Command::new("dmsetup")
        .args(["remove", name])
        .output()
        .unwrap_or_else(|e| {
            error!("Unable to unmount from device mapper: {:?}", e);
            error::unmount_error()
        });

    if !output.status.success() {
        error!("dmsetup reported an error");
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
        error::unmount_error();
    }
}

/// Unmounts image from losetup
pub fn unmount_image(name: String, error: ImageError) -> Result<(), ()> {
    info!("losetup -d {name}");
    let output = Command::new("losetup").args(["-d", &name]).output();

    let output = if let Ok(x) = output {
        x
    } else if let ImageError::ShowError = error {
        error!("Unable to run losetup {:?}", output);
        error::unmount_error();
    } else {
        error!("Unable to run losetup {:?}", output);
        return Err(());
    };

    if !output.status.success() {
        error!("losetup reported an error");
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
        error::unmount_error();
    }
    Ok(())
}

/// Function that accepts a specific device and removes it from config
fn unmount_device(device: Device, config: Option<&mut Config>) {
    let entry = format!("{}{}", config::DEVICE_NAME, device.get_entry());
    unmount_device_mapper(&entry);
    let _ = unmount_image(device.get_image_location(), ImageError::ShowError);
    if let Some(config) = config {
        config.remove_device(device.get_entry());
    }
    println!("Device {entry} unmounted");
}

/// Unmounts all devices
pub fn unmount_all() {
    let mut config = Config::read_config();
    let devices = config.iter_mut();

    for device in devices {
        unmount_device(device, None);
    }

    config.clear_devices();
}
