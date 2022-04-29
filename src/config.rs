use serde::{Deserialize, Serialize};
use std::{ffi::OsString, fs::OpenOptions, io::{ErrorKind, Read}};
use crate::error;
use std::collections::HashMap;

pub const DM_MOUNT_PATH: &str = "loopdrrem";
pub const IMAGE_MOUNT_PATH: &str = "/dev/loop50";


pub struct Device {
    pub image_mount_point: String,
    pub device_mount_point: String,
}

pub struct DeviceIterator<'a> {
    iterator: std::collections::hash_map::IterMut<'a,u32, ConfigEntry>,
}

impl Iterator for DeviceIterator<'_> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next();
        match item {
            Some(entry) => {
                Some(Device {
                    image_mount_point: entry.1.image_mount_point.clone(),
                    device_mount_point: format!("{DM_MOUNT_PATH}{}", entry.0)
                })
            },
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigEntry {
    image_file: OsString,
    /// image file mount point
    image_mount_point: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config(HashMap<u32, ConfigEntry>);

impl Config {
    pub fn read_config() -> Config {
        let mut temp = std::env::temp_dir();
        temp.push("ddr_error_mapping");
        std::fs::create_dir_all(temp.into_os_string())
                            .unwrap_or_else(|_| error::set_config_error());
        
        let file = OpenOptions::new().read(true).write(true)
                    .open("config.json");
        
        if let Err(error) = file {
            if let ErrorKind::NotFound = error.kind() {
                Config(HashMap::new())
            } else {
                error::read_config_error();
            }
        } else {
            let mut config_json = "".to_string();
            let config_status = file.unwrap().read_to_string(&mut config_json).unwrap();
            let config: Config = serde_json::from_str(&config_json).unwrap();
            config 
        }
    }

    pub fn write_device(&mut self, image_path: OsString, entry: u32, image_mount: String) {
        self.0.insert(entry,ConfigEntry { image_file: image_path, 
            image_mount_point: image_mount });
    }

    pub fn write_config(&mut self) {
        let mut temp = std::env::temp_dir();
        temp.push("ddr_error_mapping");
        temp.push("config.json");
        std::fs::write(temp, serde_json::to_string(self).unwrap().as_bytes()).unwrap();
    }
}

pub fn list_devices() {
    let config = Config::read_config();
    for devices in config.0 {
        let image = devices.1.image_file.into_string();
        if image.is_ok() {
            println!("{} mounted at {}", image.unwrap(), DM_MOUNT_PATH.to_string() + &devices.0.to_string());
        } else {
            println!("Image mounted at {}", DM_MOUNT_PATH.to_string() + &devices.0.to_string());
        }
    }
}

pub fn get_next_devices() -> u32 {
    let config = Config::read_config().0;
    let mut next_device = 0;
    for num in 0..=u32::MAX {
        if let Some(_) = config.get(&num) {
            continue;
        }
        next_device = num;
        break;
    }
    next_device
}