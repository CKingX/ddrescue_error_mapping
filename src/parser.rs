use crate::error;
use std::{ffi::OsString, fmt::Write, fs};

/// Parses ddrescue map file to dmsetup table
/// Structure of map file can be found [here](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html#Mapfile-structure)
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

    let mut prev_entry = 0;

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

        let pos = convert_to_num(&pos_string);
        let size = convert_to_num(&size_string);

        if status == '+' {
            error::handle_string_write(writeln!(
                output,
                "{}",
                create_linear(pos, size, device_name)
            ));
        } else {
            error::handle_string_write(writeln!(output, "{}", create_error(pos, size)));
        }

        // Check if sector is contiguous
        if pos != prev_entry {
            error::contiguous_error();
        } else {
            prev_entry = pos + size;
        }
    }

    output
}

/// Creates dmtable for error device
fn create_error(pos: u128, size: u128) -> String {
    format!("{} {} error", sector(pos), sector(size))
}

/// Creates dmtable for linear device
fn create_linear(pos: u128, size: u128, device: &str) -> String {
    format!(
        "{} {} linear {device} {}",
        sector(pos),
        sector(size),
        sector(pos)
    )
}

/// Divides into 512-byte sectors
fn sector(size: u128) -> u128 {
    if size % 512 != 0 {
        error::parse_error();
    } else {
        size / 512
    }
}

/// ddrescue expects pos and size to be based on C++ integer notation
/// C++ notation allows either decimal, hex (beginning with 0x), or octal (beginning with 0)
fn convert_to_num(num_string: &str) -> u128 {
    let mut num_string = num_string;

    let radix = if num_string.starts_with("0x") {
        num_string = num_string.trim_start_matches("0x");
        16
    } else if num_string.starts_with('0') {
        8
    } else {
        10
    };

    u128::from_str_radix(num_string, radix).unwrap_or_else(|_| error::convert_error())
}
