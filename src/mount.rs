use std::{ffi::{OsString}, fs, fmt::Write, process::{self, Command}};

use crate::error;
use crate::config;
use std::path::Path;

pub fn mount(image: OsString, map: OsString, block_size: u32) {
    let entry = config::get_next_devices();
    let device_name = format!("{}{}",config::DM_MOUNT_PATH,entry);
    let image_mount_path = format!("{}{}",config::IMAGE_MOUNT_PATH, 
        entry);
    let image_path = OsString::from(&image_mount_path);
    let device_mapper = &format!("{}",parse_map(map, &image_path.to_str().unwrap()));
    
    let image_mount_status = Command::new("losetup")
                            .args([&image_path, &image, &OsString::from("-b"),
                            &OsString::from(block_size.to_string())])
                            .stdin(process::Stdio::null())
                            .output().unwrap_or_else(|_| error::mount_error());
    
    if !image_mount_status.status.success() {
        error::mount_error();
    }

    if !Path::new(&image).exists() {
        ();
    }

    let dm_mount_status = Command::new("dmsetup")
                            .args(["create",&device_name,"--table",&device_mapper])
                            .stdin(process::Stdio::null())
                            .output().unwrap_or_else(|_| error::mount_error());

    if !dm_mount_status.status.success() {
        eprintln!("Unable to read image file");
        std::process::exit(error::ExitCode::FileError as i32);
    }
    
    let mut config = config::Config::read_config();
    config.write_device(image_path, entry, image_mount_path, device_name);
    config.write_config();
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