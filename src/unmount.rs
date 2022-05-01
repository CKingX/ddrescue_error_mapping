use crate::config::{self,Config};
use crate::error;

pub fn unmount(device_name: String) {
    let mut config = Config::read_config();
    let mut devices = config.iter_mut();
    let search = devices.find(|d| d.device_mount_point == device_name);

    if let Some(found_device) = search {
        todo!();
    } else {
        error::no_unmount_device(device_name);
    }
    
}