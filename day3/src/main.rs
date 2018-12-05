use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Rectangle {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rectangle {
    fn overlap(&self, other: &Self) -> Option<Rectangle> {
        if self.x1 > other.x2 || self.x2 < other.x1 {
            return None;
        }

        if self.y1 > other.y2 || self.y2 < other.y1 {
            return None;
        }

        Some(Rectangle {
            x1: self.x1.max(other.x1),
            y1: self.y1.max(other.y1),
            x2: self.x2.min(other.x2),
            y2: self.y2.min(other.y2),
        })
    }
}

fn part1() -> Result<usize, io::Error> {
    let re = Regex::new(r"^#\d+ @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)$").unwrap();
    let f = BufReader::new(File::open("input.txt")?);
    let mut rects = f
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let caps = re.captures(&line).unwrap();

            // regex matches only numbers, thus unwrap is fine
            let x1 = usize::from_str(caps.name("x").unwrap().as_str()).unwrap();
            let y1 = usize::from_str(caps.name("y").unwrap().as_str()).unwrap();
            let w = usize::from_str(caps.name("w").unwrap().as_str()).unwrap();
            let h = usize::from_str(caps.name("h").unwrap().as_str()).unwrap();
            Rectangle {
                x1,
                y1,
                x2: x1 + w - 1,
                y2: y1 + h - 1,
            }
        })
        .collect::<Vec<Rectangle>>();

    let mut overlaps = HashSet::new();
    while let Some(rect) = rects.pop() {
        for other in &rects {
            if let Some(overlap) = rect.overlap(other) {
                for x in overlap.x1..=overlap.x2 {
                    for y in overlap.y1..=overlap.y2 {
                        overlaps.insert((x, y));
                    }
                }
            }
        }
    }

    Ok(overlaps.len())
}

fn main() -> Result<(), io::Error> {
    println!("Part 1: {}", part1()?);
    Ok(())
}

#[test]
fn test_overlap() {
    let rect = Rectangle {
        x1: 4,
        y1: 4,
        x2: 7,
        y2: 7,
    };

    // left
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 2,
            y1: 5,
            x2: 3,
            y2: 6
        }),
        None
    );
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 2,
            y1: 5,
            x2: 4,
            y2: 6
        }),
        Some(Rectangle {
            x1: 4,
            y1: 5,
            x2: 4,
            y2: 6
        })
    );

    // right
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 8,
            y1: 5,
            x2: 9,
            y2: 6
        }),
        None
    );
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 7,
            y1: 5,
            x2: 8,
            y2: 6
        }),
        Some(Rectangle {
            x1: 7,
            y1: 5,
            x2: 7,
            y2: 6
        })
    );

    // top
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 5,
            y1: 2,
            x2: 6,
            y2: 3
        }),
        None
    );
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 5,
            y1: 2,
            x2: 6,
            y2: 4
        }),
        Some(Rectangle {
            x1: 5,
            y1: 4,
            x2: 6,
            y2: 4
        }),
    );

    // bottom
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 5,
            y1: 8,
            x2: 6,
            y2: 9
        }),
        None
    );
    assert_eq!(
        rect.overlap(&Rectangle {
            x1: 5,
            y1: 7,
            x2: 6,
            y2: 8
        }),
        Some(Rectangle {
            x1: 5,
            y1: 7,
            x2: 6,
            y2: 7
        }),
    );
}
