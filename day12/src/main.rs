#![feature(test)]
extern crate test;

use std::{io, mem};
use std::time::{Instant};

const INITIAL_STATE: &str = "#....##.#.#.####..#.######..##.#.########..#...##...##...##.#.#...######.###....#...##..#.#....##.##";
const PADDING: &[u8] = b"....";

fn grow(iterations: usize) -> Result<i64, io::Error> {
    let mut offset = 0;

    let mut state = INITIAL_STATE.as_bytes().to_vec();
    let mut new_state = state.clone();

    let started_at = Instant::now();

    fn sum(s: &[u8], offset: usize) -> i64 {
        let mut sum: i64 = 0;
        for (j, ch) in s.iter().enumerate() {
            if *ch == '#' as u8 {
                sum += (j as i64) - (offset as i64);
            }
        }
        sum
    }

    let mut prev_sum = 0;

    // println!("IN: {}", state);

    for n in 0..iterations {
        // grow leftwards
        if &state[0..4] != PADDING {
            offset += 4;
            state.splice(0..0, PADDING.to_vec());
        }

        // grow rightwards
        if &state[state.len()-4..] != PADDING {
            state.splice(state.len().., PADDING.to_vec());
        }

        // reset new_state to current state
        new_state.resize(state.len(), '.' as u8);
        new_state.copy_from_slice(&state);

        for (i, window) in state.windows(5).enumerate() {
            let result = match window {
                b".#.##" => Some(b'#'),
                b".#.#." => Some(b'#'),
                b"#.#.#" => Some(b'.'),
                b".####" => Some(b'.'),
                // b".#..." => Some(b'.'),
                b"#..##" => Some(b'.'),
                // b"..#.#" => Some(b'#'),
                b"#.#.." => Some(b'.'),
                b"#####" => Some(b'.'),
                // b"....#" => Some(b'.'),
                // b"...##" => Some(b'.'),
                b"..##." => Some(b'.'),
                b"##.#." => Some(b'#'),
                // b"##..#" => Some(b'.'),
                b"##..." => Some(b'#'),
                // b"..###" => Some(b'#'),
                // b".##.." => Some(b'#'),
                b"###.." => Some(b'.'),
                // b"#..#." => Some(b'.'),
                // b"##.##" => Some(b'.'),
                // b"..#.." => Some(b'#'),
                // b".##.#" => Some(b'#'),
                // b"####." => Some(b'#'),
                b"#.###" => Some(b'.'),
                b"#...#" => Some(b'#'),
                // b"###.#" => Some(b'#'),
                b"...#." => Some(b'#'),
                b".###." => Some(b'.'),
                b".#..#" => Some(b'#'),
                // b"....." => Some(b'.'),
                // b"#...." => Some(b'.'),
                // b"#.##." => Some(b'#'),
                _ => None
            };

            if let Some(result) = result {
                new_state[i+2] = result;
            }
        }

        mem::swap(&mut state, &mut new_state);

        // we should have a pattern here
        if n == 99 {
            prev_sum = sum(&state, offset);
        }

        if n == 100 {
            let diff = sum(&state, offset) - prev_sum;
            prev_sum += ((iterations - n) as i64 * diff) as i64;
            break;
        }

        // println!("{:02}: {} ({})", n + 1, std::str::from_utf8(&state).unwrap(), diff);
    }

    if prev_sum == 0 {
        prev_sum = sum(&state, offset);
    }

    let elapsed = started_at.elapsed();
    let elapsed = elapsed.as_secs() as f64
        + elapsed.subsec_nanos() as f64 * 1e-9;
    println!("Finished in {}s", elapsed);

    Ok(prev_sum)
}

fn main() -> Result<(), io::Error> {
    let part1 = grow(20)?;
    assert_eq!(part1, 2444);
    println!("Part 1: {}", part1);

    let part2 = grow(50_000_000_000)?;
    assert_eq!(part2, 750000000697);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::mem;

    #[bench]
    fn bench_first(b: &mut Bencher) {
        let mut state = b"#....##.#.#.####..#.######..##.#.########..#...##...##...##.#.#...######.###....#...##..#.#....##.##".to_vec();
        let mut new_state = state.clone();

        b.iter(|| {
//            new_state.resize(state.len(), '.' as u8);
            new_state.copy_from_slice(&state);

            for (i, window) in state.windows(5).enumerate() {
                let result = match window {
                    b".#.##" => Some(b'#'),
                    b".#.#." => Some(b'#'),
                    b"#.#.#" => Some(b'.'),
                    b".####" => Some(b'.'),
                    b".#..." => Some(b'.'),
                    b"#..##" => Some(b'.'),
                    b"..#.#" => Some(b'#'),
                    b"#.#.." => Some(b'.'),
                    b"#####" => Some(b'.'),
                    b"....#" => Some(b'.'),
                    b"...##" => Some(b'.'),
                    b"..##." => Some(b'.'),
                    b"##.#." => Some(b'#'),
                    b"##..#" => Some(b'.'),
                    b"##..." => Some(b'#'),
                    b"..###" => Some(b'#'),
                    b".##.." => Some(b'#'),
                    b"###.." => Some(b'.'),
                    b"#..#." => Some(b'.'),
                    b"##.##" => Some(b'.'),
                    b"..#.." => Some(b'#'),
                    b".##.#" => Some(b'#'),
                    b"####." => Some(b'#'),
                    b"#.###" => Some(b'.'),
                    b"#...#" => Some(b'#'),
                    b"###.#" => Some(b'#'),
                    b"...#." => Some(b'#'),
                    b".###." => Some(b'.'),
                    b".#..#" => Some(b'#'),
                    b"....." => Some(b'.'),
                    b"#...." => Some(b'.'),
                    b"#.##." => Some(b'#'),
                    _ => None
                };

                if let Some(result) = result {
                    new_state[i+2] = result;
                }
            }

            mem::swap(&mut state, &mut new_state);
        });
    }

    #[bench]
    fn bench_second(b: &mut Bencher) {
        let mut state = b"#....##.#.#.####..#.######..##.#.########..#...##...##...##.#.#...######.###....#...##..#.#....##.##".to_vec();

        b.iter(|| {
            let flip : Vec<usize> = state.windows(5).enumerate().filter_map(|(i, window)| {
                let result = match window {
                    b".#.##" => Some(b'#'),
                    b".#.#." => Some(b'#'),
                    b"#.#.#" => Some(b'.'),
                    b".####" => Some(b'.'),
                    b".#..." => Some(b'.'),
                    b"#..##" => Some(b'.'),
                    b"..#.#" => Some(b'#'),
                    b"#.#.." => Some(b'.'),
                    b"#####" => Some(b'.'),
                    b"....#" => Some(b'.'),
                    b"...##" => Some(b'.'),
                    b"..##." => Some(b'.'),
                    b"##.#." => Some(b'#'),
                    b"##..#" => Some(b'.'),
                    b"##..." => Some(b'#'),
                    b"..###" => Some(b'#'),
                    b".##.." => Some(b'#'),
                    b"###.." => Some(b'.'),
                    b"#..#." => Some(b'.'),
                    b"##.##" => Some(b'.'),
                    b"..#.." => Some(b'#'),
                    b".##.#" => Some(b'#'),
                    b"####." => Some(b'#'),
                    b"#.###" => Some(b'.'),
                    b"#...#" => Some(b'#'),
                    b"###.#" => Some(b'#'),
                    b"...#." => Some(b'#'),
                    b".###." => Some(b'.'),
                    b".#..#" => Some(b'#'),
                    b"....." => Some(b'.'),
                    b"#...." => Some(b'.'),
                    b"#.##." => Some(b'#'),
                    _ => None
                };

                result.map(|_| i)
            }).collect();

            for i in flip {
                state[i+2] = if state[i+2] == '#' as u8 {
                    '.' as u8
                } else { '#' as u8 };
            }
        });
    }
}