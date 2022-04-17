use text_io::scan;
use std::{env, fs, process};

static CONVERT_ERROR: &str = "Could not convert hex to decimal";
static FILE_ERROR: &str = "You must put the path to map file";
static DEVICE_ERROR: &str = "No device path given";
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 3 {
        println!("ddrescue_error_mapping <ddrescue log> <device path>");
        process::exit(0);
    }
    let filename;
    let device_name;
    {
        let mut options = args.into_iter();
        filename = options.nth(1).expect(FILE_ERROR);
        device_name = options.nth(0).expect(DEVICE_ERROR);
    }
    
    let contents= fs::read_to_string(filename)
                    .unwrap_or_else(|_| panic!("File may be corrupted"));

    let mut file_line = contents.lines()
                        .filter(|s| !s.is_empty() && !s.contains("#"))
                        .map(|s| s.trim())
                        .skip(1);

    let mut output: Vec<String> = Vec::new();

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

    output.into_iter().for_each(|n| println!("{}", n));
}

    fn create_error(pos: u64, size: u64) -> String {
        format!("{} {} error", pos/512, size/512)
    }

    fn create_linear(pos: u64, size: u64, device: &str) -> String {
        format!("{} {} linear {} {}", pos/512, size/512, device, pos/512)
    }