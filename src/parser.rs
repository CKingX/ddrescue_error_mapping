use crate::error;
use std::{ffi::OsString, fmt::Write, fs};

pub fn parse_map(map_path: &OsString, device_name: &str) -> String {
    let mut output = String::new();
    let contents = fs::read_to_string(map_path.clone()).unwrap_or_else(|error| {
        error::check_io_error(
            error,
            map_path.clone().into_string().unwrap_or_default(),
            crate::error::FileType::MapFile,
        )
    });

    let file_line = contents
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.contains('#'))
        .skip(1);

    for line in file_line {
        let mut map_line = line.split_ascii_whitespace();
        let pos_string = map_line.next().expect(error::PARSE_ERROR).to_string();
        let size_string = map_line.next().expect(error::PARSE_ERROR).to_string();
        let status = map_line
            .next()
            .expect(error::PARSE_ERROR)
            .chars()
            .next()
            .unwrap();

        let pos = u64::from_str_radix(pos_string.trim_start_matches("0x"), 16)
            .expect(error::CONVERT_ERROR);
        let size = u64::from_str_radix(size_string.trim_start_matches("0x"), 16)
            .expect(error::CONVERT_ERROR);

        if status == '+' {
            error::handle_string_write(writeln!(
                output,
                "{}",
                create_linear(pos, size, device_name)
            ));
        } else {
            error::handle_string_write(writeln!(output, "{}", create_error(pos, size)));
        }
    }

    output
}

/// Creates dmtable for error device
fn create_error(pos: u64, size: u64) -> String {
    format!("{} {} error", sector(pos), sector(size))
}

/// Creates dmtable for linear device
fn create_linear(pos: u64, size: u64, device: &str) -> String {
    format!(
        "{} {} linear {device} {}",
        sector(pos),
        sector(size),
        sector(pos)
    )
}

fn sector(size: u64) -> u64 {
    if size % 512 != 0 {
        error::parse_error();
    } else {
        size / 512
    }
}
