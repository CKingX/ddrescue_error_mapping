use serde::{Deserialize, Serialize};
use std::{ffi::OsString, fs::OpenOptions, io::{ErrorKind, Read}};
use crate::error::{self, set_config_error};
use std::collections::HashMap;

pub const DM_MOUNT_PATH: &str = "ddrm";
pub const IMAGE_MOUNT_PATH: &str = "/dev/loop50";
pub const CONFIG_FOLDER: &str = "ddr_mount";


pub struct Device {
    pub image_file_path: OsString,
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
                    image_file_path: entry.1.image_file.clone(),
                    device_mount_point: entry.1.dm_mount_point.clone(),
                })
            },
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigEntry {
    image_file: OsString,
    /// image file mount point
    image_mount_point: String,
    dm_mount_point: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config(HashMap<u32, ConfigEntry>);

impl Config {
    pub fn read_config() -> Config {
        let mut temp = std::env::temp_dir();
        temp.push(CONFIG_FOLDER);
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
            let mut config_entries = String::new();
            let file_size = file.unwrap().read_to_string(&mut config_entries)
                .unwrap_or_else(|_| set_config_error());
            if file_size == 0 {
                config_entries = "{}".to_string();
            }
            let config: Config = serde_json::from_str(&config_entries).unwrap();
            dbg!(config.clone());
            config 
        }
    }

    pub fn iter_mut(&mut self) -> DeviceIterator {
        DeviceIterator { iterator: self.0.iter_mut() }
    }

    pub fn write_device(&mut self, image_path: OsString, entry: u32, image_mount: String, dm_mount_point: String) {
        self.0.insert(entry,ConfigEntry {
            image_file: image_path, 
            image_mount_point: image_mount,
            dm_mount_point,});
    }

    pub fn write_config(&mut self) {
        let mut temp = std::env::temp_dir();
        temp.push(CONFIG_FOLDER);
        temp.push("config.json");
        std::fs::write(temp, serde_json::to_string(self).unwrap().as_bytes()).unwrap();
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        self.write_config();
    }
}

pub fn list_devices() {
    let mut config = Config::read_config();

    for devices in config.iter_mut() {
        let image = devices.image_file_path.to_str();
        if image.is_some() {
            println!("{} mounted at {}", image.unwrap(), devices.device_mount_point);
        } else {
            println!("Image mounted at {}", devices.device_mount_point);
        }
    }
}

pub fn get_next_devices() -> u32 {
    let config = &Config::read_config().0;
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