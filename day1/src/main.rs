use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

fn main() -> Result<(), io::Error> {
    // Note: we are ignoring possible errors ;-)
    let result: i32 = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(Result::ok)
        .map(|l| i32::from_str(&l))
        .filter_map(Result::ok)
        .sum();

    println!("Result: {}", result);

    Ok(())
}
