use std::process::Command;

use crate::config::{self,Config};
use crate::error;

pub fn unmount(device_name: String) {
    let mut config = Config::read_config();
    let mut devices = config.iter_mut();
    let search = devices.find(|d| d.device_mount_point == device_name);

    if let Some(found_device) = search {
        let entry = format!("{}{}",config::DEVICE_NAME, found_device.get_entry());
        unmount_device_mapper(entry);
        unmount_image(found_device.get_image_location());
        config.remove_device(found_device.get_entry());
        println!("Device {device_name} unmounted");
    } else {
        error::no_unmount_device(device_name);
    }
}

pub fn unmount_device_mapper(name: String) {
    let output = Command::new("dmsetup")
                .args(["remove", &name])
                .output().unwrap_or_else(|_| error::unmount_error());
    
    if !output.status.success() {
        error::unmount_error();
    }
}

pub fn unmount_image(name: String) {
    let output = Command::new("losetup")
                .args(["-d", &name])
                .output().unwrap_or_else(|_| error::unmount_error());
    
    if !output.status.success() {
        error::unmount_error();
    }
}