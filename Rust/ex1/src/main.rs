use regex::Regex;
use std::default;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn parse_line(line: &str, regex: &Regex) -> usize {
    if let Some(caps) = regex.captures(line) {
        return match caps.get(0).unwrap().as_str() {
            "0" | "zero" | "orez" => 0,
            "1" | "one" | "eno" => 1,
            "2" | "two" | "owt" => 2,
            "3" | "three" | "eerht" => 3,
            "4" | "four" | "ruof" => 4,
            "5" | "five" | "evif" => 5,
            "6" | "six" | "xis" => 6,
            "7" | "seven" | "neves" => 7,
            "8" | "eight" | "thgie" => 8,
            "9" | "nine" | "enin" => 9,
            _default => panic!(),
        };
    }
    panic!();
}

fn main() {
    let in_file = File::open("./ex1-2.txt").unwrap();
    let reader = BufReader::new(in_file);

    let re_forward = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_reverse = Regex::new(r"\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let sum: usize = reader
        .lines()
        .map(|line| {
            let forward = line.unwrap();
            let reversed: String = forward.chars().rev().collect();

            let a = parse_line(&forward, &re_forward);
            let b = parse_line(&reversed, &re_reverse);

            a * 10 + b
        })
        .sum();

    println!("sum: {}", sum);
}
