use crate::error::{self, set_config_error};
use indexmap::IndexMap;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::os::unix::prelude::*;
use std::{
    ffi::OsString,
    fs::File,
    io::{ErrorKind, Read},
};

pub const DEVICE_NAME: &str = "ddrm";
pub const CONFIG_FOLDER: &str = "ddr-mount";
pub const DM_LOCATION: &str = "/dev/mapper/";

pub struct Device {
    pub image_file_path: OsString,
    pub device_mount_point: String,
    entry: u32,
    image_mount: ImageLocation,
}

#[derive(Clone)]
pub struct ImageLocation {
    image_path: String,
}

impl Device {
    pub fn get_entry(&self) -> u32 {
        self.entry
    }

    pub fn get_image_location(&self) -> String {
        self.image_mount.image_path.clone()
    }
}

pub struct DeviceIterator<'a> {
    iterator: indexmap::map::IterMut<'a, u32, ConfigEntry>,
}

impl Iterator for DeviceIterator<'_> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next();
        match item {
            Some(entry) => Some(Device {
                image_file_path: entry.1.image_file.clone(),
                device_mount_point: entry.1.dm_mount_point.clone(),
                entry: *entry.0,
                image_mount: ImageLocation {
                    image_path: entry.1.image_mount_point.clone(),
                },
            }),
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
pub struct Config(#[serde(with = "indexmap::serde_seq")] IndexMap<u32, ConfigEntry>);

impl Config {
    pub fn read_config() -> Config {
        let mut temp = std::env::temp_dir();
        temp.push(CONFIG_FOLDER);
        std::fs::create_dir_all(&temp).unwrap_or_else(|e| {
            error!("Unable to create configuration {:?}", e);
            error::set_config_error()
        });
        temp.push("config.json");

        info!("Configuration location: {:?}", temp);

        let file = File::options().read(true).mode(0o664).open(&temp);

        if let Err(error) = file {
            if let ErrorKind::NotFound = error.kind() {
                Config(IndexMap::new())
            } else {
                error!("Configuration open error {:?}", error);
                error::read_config_error();
            }
        } else {
            let mut config_entries = String::new();
            let file_size = file
                .unwrap()
                .read_to_string(&mut config_entries)
                .unwrap_or_else(|e| {
                    error!("Unable to convert configuration to file, {:?}", e);
                    set_config_error()
                });

            if file_size == 0 {
                config_entries = "{}".to_string();
            }
            let config: Config = serde_json::from_str(&config_entries).unwrap_or_else(|e| {
                error!("Unable to parse error {:?}", e);
                set_config_error()
            });

            config
        }
    }

    pub fn iter_mut(&mut self) -> DeviceIterator {
        DeviceIterator {
            iterator: self.0.iter_mut(),
        }
    }

    pub fn write_device(
        &mut self,
        image_path: OsString,
        entry: u32,
        image_mount: String,
        dm_mount_point: String,
    ) {
        self.0.insert(
            entry,
            ConfigEntry {
                image_file: image_path,
                image_mount_point: image_mount,
                dm_mount_point,
            },
        );
    }

    pub fn write_config(&mut self) {
        let mut temp = std::env::temp_dir();
        temp.push(CONFIG_FOLDER);
        temp.push("config.json");
        std::fs::write(temp, serde_json::to_string(self).unwrap().as_bytes()).unwrap_or_else(|e| {
            error!("Unable to write configuration {:?}", e);
            set_config_error()
        });

        info!("Configuration file written");
    }

    pub fn remove_device(&mut self, entry: u32) {
        if self.0.remove(&entry).is_none() {
            error::set_config_error();
        }
    }

    pub fn clear_devices(&mut self) {
        self.0.clear();
        info!("All devices cleared");
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        self.0.sort_keys();
        self.write_config();
    }
}

pub fn list_devices() {
    let mut config = Config::read_config();

    let mut max_size = 0;

    for device in config.iter_mut() {
        let image = device.image_file_path.to_string_lossy();
        let count = image.chars().count();
        if count > max_size {
            max_size = count;
        }
    }
    max_size += 1;

    for device in config.iter_mut() {
        let image = if let Some(name) = device.image_file_path.to_str() {
            let count = name.chars().count();
            name.to_string() + &" ".repeat(max_size - count)
        } else {
            let name = "Unknown Image";
            name.to_string() + &" ".repeat(max_size - name.chars().count())
        };

        println!("{image} => {DM_LOCATION}{}", device.device_mount_point);
    }

    std::mem::forget(config);
}

pub fn get_next_devices() -> u32 {
    let config = &Config::read_config().0;
    let mut next_device = 0;
    for num in 1..=u32::MAX {
        if config.get(&num).is_some() {
            continue;
        }
        next_device = num;
        break;
    }
    next_device
}
