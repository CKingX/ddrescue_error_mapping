#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use ddr_mount::parse_map_string;

    #[test]
    fn test1() {
        let input = include_str!("./test1.txt");
        let output = parse_map_string(&OsString::from("test1.txt"), input, "/dev/loop##");

        assert_eq!(output, include_str!("./test1output.txt"));
    }

    #[test]
    fn test2() {
        let input = include_str!("./test2.txt");
        let output = parse_map_string(&OsString::from("test2.txt"), input, "/dev/loop##");

        assert_eq!(output, include_str!("./test2output.txt"));
    }

    #[test]
    fn test3() {
        let input = include_str!("./test3.txt");
        let output = parse_map_string(&OsString::from("test3.txt"), input, "/dev/loop##");

        assert_eq!(output, include_str!("./test3output.txt"));
    }
}
