use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn part1() -> Result<i32, io::Error> {
    // Note: we are ignoring possible errors ;-)
    let result: i32 = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(Result::ok)
        .map(|l| i32::from_str(&l))
        .filter_map(Result::ok)
        .sum();

    Ok(result)
}

fn part2() -> Result<i32, io::Error> {
    let nrs: Vec<i32> = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(Result::ok)
        .map(|l| i32::from_str(&l))
        .filter_map(Result::ok)
        .collect();

    let mut results = HashSet::new();
    let mut total = 0;
    for nr in nrs.into_iter().cycle() {
        total += nr;
        if results.contains(&total) {
            break;
        }
        results.insert(total);
    }

    return Ok(total);
}

fn main() -> Result<(), io::Error> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);
    Ok(())
}
