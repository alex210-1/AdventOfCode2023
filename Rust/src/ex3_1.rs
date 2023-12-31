use std::io::BufRead;
use std::{fs::File, io::BufReader};

use regex::Regex;

pub fn run() {
    let re = Regex::new(r"\d+").unwrap();

    let mut symbols = [[false; 150]; 150];

    let mut sum = 0;

    let in_file = File::open("./ex3-1.txt").unwrap();
    let reader = BufReader::new(in_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                symbols[y][x] = true;
            }
        }
    }

    let is_symbol = |x: i32, y: i32| {
        if x < 0 || x > 150 || y < 0 || y > 150 {
            return false;
        }
        symbols[y as usize][x as usize]
    };

    for (y, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let value: i32 = m.as_str().parse().unwrap();

            let adjacent = (m.start() as i32 - 1..=m.end() as i32)
                .map(|x| is_symbol(x, y as i32 - 1) || is_symbol(x, y as i32 + 1))
                .any(|b| b);

            if adjacent
                || is_symbol(m.start() as i32 - 1, y as i32)
                || is_symbol(m.end() as i32, y as i32)
            {
                sum += value;
            }
        }
    }

    println!("\nsum: {}", sum);
}
