use std::process::Command;

use crate::config::{self, Config, Device};
use crate::error;

pub enum ImageError {
    ShowError,
    HideError,
}

/// Unmounts a device
pub fn unmount(device_name: String) {
    let mut config = Config::read_config();
    let mut devices = config.iter_mut();
    let search = devices.find(|d| d.device_mount_point == device_name);

    if let Some(device) = search {
        unmount_device(device, Some(&mut config));
    } else {
        error::no_unmount_device(device_name);
    }
}

/// Unmounts device from device mapper
pub fn unmount_device_mapper(name: String) {
    let output = Command::new("dmsetup")
        .args(["remove", &name])
        .output()
        .unwrap_or_else(|_| error::unmount_error());

    if !output.status.success() {
        error::unmount_error();
    }
}

/// Unmounts image from losetup
pub fn unmount_image(name: String, error: ImageError) -> Result<(), ()> {
    let output = Command::new("losetup").args(["-d", &name]).output();

    let output = if let Ok(x) = output {
        x
    } else if let ImageError::ShowError = error {
        error::unmount_error();
    } else {
        return Err(());
    };

    if !output.status.success() {
        error::unmount_error();
    }
    Ok(())
}

/// Function that accepts a specific device and removes it from config
fn unmount_device(device: Device, config: Option<&mut Config>) {
    let entry = format!("{}{}", config::DEVICE_NAME, device.get_entry());
    unmount_device_mapper(entry.clone());
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
