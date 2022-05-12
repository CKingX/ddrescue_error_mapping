use crate::error::{self, Token};
use colored::Colorize;
use std::{borrow::Cow, ffi::OsString, fmt::Write, fs};

struct Number<'a> {
    pos: (u128, &'a str),
    size: (u128, &'a str),
}

struct Line<'a> {
    filename: Cow<'a, str>,
    line_num: usize,
    line: &'a str,
}

/// Reads the map file and send it to parser
pub fn parse_map(map_path: &OsString, device_name: &str) -> String {
    let contents = fs::read_to_string(map_path.clone()).unwrap_or_else(|error| {
        error::check_io_error(
            error,
            map_path.clone().into_string().unwrap_or_default(),
            crate::error::FileType::MapFile,
        )
    });

    parse_map_string(map_path, &contents, device_name)
}

/// Creates dmtable for error device
fn create_error(contents: Number, line: &Line) -> String {
    let Number {
        pos: (pos, pos_string),
        size: (size, size_string),
    } = contents;
    format!(
        "{} {} error",
        sector(pos, || report_error(
            line,
            line.line.find(pos_string).unwrap(),
            pos_string,
            error::POSITION_SECTOR_ERROR
        )),
        sector(size, || report_error(
            line,
            line.line.rfind(size_string).unwrap(),
            size_string,
            error::SIZE_SECTOR_ERROR
        ))
    )
}

/// Creates dmtable for linear device
fn create_linear(contents: Number, device: &str, line: &Line) -> String {
    let Number {
        pos: (pos, pos_string),
        size: (size, size_string),
    } = contents;
    let pos_sector = sector(pos, || {
        report_error(
            line,
            line.line.find(pos_string).unwrap(),
            pos_string,
            error::POSITION_SECTOR_ERROR,
        )
    });
    let size_sector = sector(size, || {
        report_error(
            line,
            line.line.rfind(size_string).unwrap(),
            size_string,
            error::SIZE_SECTOR_ERROR,
        )
    });
    format!("{pos_sector} {size_sector} linear {device} {pos_sector}")
}

/// Parses ddrescue map file to dmsetup table
/// Structure of map file can be found [here](https://www.gnu.org/software/ddrescue/manual/ddrescue_manual.html#Mapfile-structure)
pub fn parse_map_string(filename: &OsString, contents: &str, device_name: &str) -> String {
    let mut output = String::new();

    let mut file_line = contents
        .lines()
        .enumerate()
        .map(|s| (s.0, s.1.trim()))
        .filter(|s| !s.1.is_empty() && !s.1.contains('#'));

    verify_status_line(&mut file_line, filename.to_string_lossy());

    let mut prev_entry = 0;

    #[allow(clippy::redundant_closure)]
    for (line_number, line) in file_line {
        let line = Line {
            filename: filename.to_string_lossy(),
            line_num: line_number,
            line,
        };

        let mut map_line = line.line.split_ascii_whitespace();
        let pos_string = map_line
            .next()
            .unwrap_or_else(|| {
                report_error(&line, 0, line.line, error::NO_POSITION_ERROR);
            })
            .to_string();
        let size_string = map_line
            .next()
            .unwrap_or_else(|| {
                report_error(&line, 0, line.line, error::NO_SIZE_ERROR);
            })
            .to_string();
        let status = map_line.next().unwrap_or_else(|| {
            report_error(&line, 0, line.line, error::NO_STATUS_ERROR);
        });

        let status = status.parse::<char>().unwrap_or_else(|_| {
            report_error(
                &line,
                line.line.rfind(status).unwrap(),
                status,
                error::UNKNOWN_MAP_STATUS_ERROR,
            )
        });

        let pos = convert_to_num(&pos_string, || {
            report_error(
                &line,
                line.line.find(&pos_string).unwrap(),
                &pos_string,
                &error::convert_error_string(Token::Pos),
            )
        });

        let location = if size_string == pos_string {
            let x = line.line.find(&size_string).unwrap() + size_string.len();
            x + line.line[x..].find(&size_string).unwrap()
        } else {
            line.line.find(&size_string).unwrap()
        };

        let size = convert_to_num(&size_string, || {
            report_error(
                &line,
                location,
                &size_string,
                &error::convert_error_string(Token::Size),
            )
        });

        let number = Number {
            pos: (pos, &pos_string),
            size: (size, &size_string),
        };

        match status {
            '+' => error::handle_string_write(writeln!(
                output,
                "{}",
                create_linear(number, device_name, &line)
            )),
            '?' | '*' | '/' | '-' => {
                error::handle_string_write(writeln!(output, "{}", create_error(number, &line)))
            }
            x => {
                let x = &x.to_string();
                report_error(
                    &line,
                    line.line.rfind(x).unwrap(),
                    x,
                    error::UNKNOWN_MAP_STATUS_ERROR,
                );
            }
        }

        // Check if sector is contiguous
        if pos != prev_entry {
            if prev_entry == 0 {
                report_error(
                    &line,
                    line.line.find(&pos_string).unwrap(),
                    &pos_string,
                    error::START_NONZERO_ERROR,
                );
            } else {
                report_error(
                    &line,
                    line.line.find(&pos_string).unwrap(),
                    &pos_string,
                    &error::CONTIGUOUS_ERROR
                        .replace("{pos}", &pos.to_string())
                        .replace("{size}", &prev_entry.to_string()),
                );
            }
        } else {
            prev_entry = pos + size;
        }
    }

    output
}
/// Divides into 512-byte sectors
fn sector(size: u128, error: impl FnOnce()) -> u128 {
    if size % 512 != 0 {
        error();
        unreachable!()
    } else {
        size / 512
    }
}

/// ddrescue expects pos and size to be based on C++ integer notation
/// C++ notation allows either decimal, hex (beginning with 0x), or octal (beginning with 0)
fn convert_to_num(num_string: &str, error: impl FnOnce()) -> u128 {
    let mut num_string = num_string;

    let radix = if num_string.starts_with("0x") {
        num_string = num_string.trim_start_matches("0x");
        16
    } else if num_string.starts_with('0') {
        8
    } else {
        10
    };

    u128::from_str_radix(num_string, radix).unwrap_or_else(|_| {
        error();
        unreachable!()
    })
}

/// Prints error in the same way cargo does
fn report_error(line: &Line, parse_start: usize, token: &str, message: &str) -> ! {
    let Line {
        filename,
        line_num,
        line,
    } = line;

    let seperator = "|".blue().bold();

    let padding = " ".repeat(line_num.to_string().len());

    eprintln!("{padding}{} {filename}", "-->".blue().bold());
    eprintln!(" {padding} {seperator}");
    eprintln!(" {} {seperator} {line}", line_num.to_string().blue().bold());

    eprintln!(
        " {padding} {seperator} {}{}",
        " ".repeat(parse_start),
        "^".repeat(token.len()).red().bold()
    );
    error::print_error(message);
    error::parse_error(false)
}

fn verify_status_line<'a, T>(lines: &mut T, filename: Cow<'a, str>)
where
    T: Iterator<Item = (usize, &'a str)>,
{
    let (line_number, line) = lines.next().unwrap_or_else(|| {
        error::print_error(error::EMPTY_MAP_ERROR);
        error::parse_error(false)
    });

    let mut contents = line.split_ascii_whitespace();

    let line_content = Line {
        filename,
        line_num: line_number,
        line,
    };

    let current_pos = contents
        .next()
        .unwrap_or_else(|| report_error(&line_content, 0, line, error::NO_CURRENT_POSITION_ERROR));

    convert_to_num(current_pos, || {
        report_error(
            &line_content,
            line.find(current_pos).unwrap(),
            current_pos,
            &error::convert_error_string(Token::CurrentPos),
        )
    });

    let current_status = contents
        .next()
        .unwrap_or_else(|| report_error(&line_content, 0, line, error::NO_CURRENT_STATUS_ERROR));

    let location = if current_status == current_pos {
        let x = line.find(current_status).unwrap() + current_status.len();
        x + line[x..].find(current_status).unwrap()
    } else {
        line.find(current_status).unwrap()
    };

    let current_status = current_status.parse::<char>().unwrap_or_else(|_| {
        report_error(
            &line_content,
            location,
            current_status,
            error::UNKNOWN_CURRENT_STATUS_ERROR,
        )
    });

    match current_status {
        '?' | '*' | '/' | '-' | 'F' | 'G' | '+' => (),
        x => report_error(
            &line_content,
            line.rfind(x).unwrap(),
            &x.to_string(),
            error::UNKNOWN_CURRENT_STATUS_ERROR,
        ),
    }

    let current_pass = contents.next();

    match current_pass {
        Some(x) => {
            x.parse::<u8>().unwrap_or_else(|_| {
                report_error(
                    &line_content,
                    line.rfind(x).unwrap(),
                    x,
                    error::UNKNOWN_CURRRENT_PHASE_ERROR,
                )
            });
        }
        None => (),
    };
}
