use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::{Dedup, Itertools};
use regex::Regex;

pub fn run() {
    let re = Regex::new(r"\d+").unwrap();

    let mut symbols = [[-1; 150]; 150];
    let mut sum = 0;

    let in_file = File::open("./ex3-1.txt").unwrap();
    let reader = BufReader::new(in_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for (y, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let value: i32 = m.as_str().parse().unwrap();

            for x in m.range() {
                symbols[y][x] = value;
            }
        }
    }

    let get_part = |x: i32, y: i32| {
        if x < 0 || x > 150 || y < 0 || y > 150 {
            return -1;
        }
        symbols[y as usize][x as usize]
    };

    let neighbours = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let parts: Vec<i32> = neighbours
                    .iter()
                    .map(|(rx, ry)| get_part(x as i32 + rx, y as i32 + ry))
                    .filter(|part| *part != -1)
                    .dedup()
                    .collect();

                if parts.len() == 2 {
                    sum += parts[0] * parts[1];
                }
            }
        }
    }

    println!("\nsum: {}", sum);
}
