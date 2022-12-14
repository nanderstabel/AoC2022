use std::{fmt, fs::File, io::Read, path::Path, str::FromStr};

pub const SINGLELINE: &str = "\n";
pub const MULTILINE: &str = "\n\n";

#[macro_export]
macro_rules! output {
    ($part1:expr, $part2:expr) => {
        println!("Part 1: {}\nPart 2: {}", $part1, $part2);
    };
    ($output:expr) => {
        println!("Part 1: {}\nPart 2: {}", $output.0, $output.1);
    };
}

pub fn read<T: FromStr>(filename: impl AsRef<Path>, delim: &str) -> Vec<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    let mut file = File::open(filename).expect("no such file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("expect able to read input file");

    buf.split(delim).map(|s| s.parse::<T>().unwrap()).collect()
}

pub fn read_input<T: FromStr>(filename: impl AsRef<Path>) -> T
where
    <T as FromStr>::Err: fmt::Debug,
{
    let mut file = File::open(filename).expect("no such file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("expect able to read input file");

    buf.parse::<T>().unwrap()
}
