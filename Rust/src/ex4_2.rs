use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;
use regex::Regex;

fn to_nums(s: &str) -> Vec<i32> {
    let re = Regex::new(r"\d+").unwrap(); // trololo

    re.find_iter(s)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn get_score(card: &str) -> usize {
    let content = card.split(":").skip(1).exactly_one().unwrap();
    let parts: Vec<&str> = content.split("|").collect();

    let having = to_nums(parts[0]);
    let winning = to_nums(parts[1]);

    return having.iter().filter(|val| winning.contains(val)).count();
}

pub fn run() {
    let in_file = File::open("./ex4-1.txt").unwrap();
    let reader = BufReader::new(in_file);

    let mut scores: Vec<(usize, usize)> = reader
        .lines()
        .map(|l| (1, get_score(&l.unwrap())))
        .collect();

    for index in 0..scores.len() {
        let (n, score) = scores[index];

        println!("i: {}, n: {}, s: {}", index, n, score);

        for i in 0..score {
            scores[index + i + 1].0 += n;
        }
    }

    let sum: usize = scores.iter().map(|(n, _)| n).sum();

    println!("\nsum: {}", sum);
}
