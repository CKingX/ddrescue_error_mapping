use text_io::scan;
use std::{env, fs, process};
#[cfg(target_os = "windows")]
extern crate static_vcruntime;

const CONVERT_ERROR: &str = "Could not convert hex to decimal";
const NO_PATH: &str = "No path to file given";
const DEVICE_ERROR: &str = "No device path given";
const HELP: &str = "ddr_error_mapping <ddrescue log> <device path> [sector size (512 default)]\n
--help -h Shows this page\n
--version -v Shows the version";
const FILE_ERROR: &str = "Unable to either open or parse file";
const VERSION: &str = "ddr_error_mapping  0.5.1";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 3 {
        if let Some(arg) = args.iter().nth(1) {
            if arg.contains("--version") || arg.contains("-v") {
                println!("{VERSION}");
            }
        }  else {
            println!("{HELP}");
        }
        process::exit(0);
    }
    let filename;
    let device_name;

    let mut options = args.into_iter();
    filename = options.nth(1).expect(NO_PATH);
    device_name = options.nth(0).expect(DEVICE_ERROR);
    
    let contents= fs::read_to_string(filename)
                    .unwrap_or_else(|_| panic!("{FILE_ERROR}"));

    let mut file_line = contents.lines()
                        .filter(|s| !s.is_empty() && !s.contains("#"))
                        .map(|s| s.trim())
                        .skip(1);

    let mut output: Vec<String> = if let (_, Some(capacity)) = file_line.size_hint() {
        Vec::with_capacity(capacity)
    } else {
        Vec::new()
    };

    while let Some(line) = file_line.next() {
        let pos_string: String;
        let pos: u64;
        let size: u64;
        let size_string: String;
        let status: char;
        scan!(line.bytes() => "{}  {}  {}", pos_string,size_string,status);
        pos = u64::from_str_radix(pos_string.trim_start_matches("0x"),16)
                .expect(CONVERT_ERROR);
        size = u64::from_str_radix(size_string.trim_start_matches("0x"),16)
                .expect(CONVERT_ERROR);
        
        if status == '+' {
            output.push(create_linear(pos,size, &device_name));
        } else {
            output.push(create_error(pos, size));
        }
    }

    output.into_iter().for_each(|n| println!("{n}"));
}

fn create_error(pos: u64, size: u64) -> String {
    format!("{} {} error", pos/512, size/512)
}

fn create_linear(pos: u64, size: u64, device: &str) -> String {
    format!("{} {} linear {device} {}", pos/512, size/512, pos/512)
}