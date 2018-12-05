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

fn part2() -> Result<Option<String>, io::Error> {
    struct ID(String);

    impl PartialEq for ID {
        fn eq(&self, other: &ID) -> bool {
            let dist: usize = self
                .0
                .chars()
                .zip(other.0.chars())
                .map(|(l, r)| if l != r { 1 } else { 0 })
                .sum();
            dist == 1
        }
    }

    let mut ids: Vec<ID> = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(Result::ok)
        .map(ID)
        .collect();

    while let Some(id) = ids.pop() {
        for other in &ids {
            if &id == other {
                let common =
                    id.0.chars()
                        .zip(other.0.chars())
                        .filter_map(|(l, r)| if l == r { Some(l) } else { None })
                        .collect::<String>();
                return Ok(Some(common));
            }
        }
    }

    Ok(None)
}

fn main() -> Result<(), io::Error> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?.unwrap_or_default());
    Ok(())
}
