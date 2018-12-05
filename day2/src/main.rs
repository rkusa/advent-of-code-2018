use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part1() -> Result<i32, io::Error> {
    let f = BufReader::new(File::open("input.txt")?);
    let mut multiples = HashMap::new();
    for line in f.lines() {
        let mut letters = HashMap::new();
        for ch in line?.chars() {
            let count = letters.entry(ch).or_insert(0);
            *count += 1;
        }
        let mut counts: Vec<i32> = letters
            .into_iter()
            .filter_map(|(_, n)| if n > 1 { Some(n) } else { None })
            .collect();
        counts.dedup();
        for count in counts {
            if count > 1 {
                let count = multiples.entry(count).or_insert(0);
                *count += 1;
            }
        }
    }

    Ok(multiples.values().product())
}

fn main() -> Result<(), io::Error> {
    println!("Part 1: {}", part1()?);
    Ok(())
}
