use std::cmp::max;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let in_file = File::open("./ex2-1.txt").unwrap();
    let reader = BufReader::new(in_file);

    let mut sum = 0;

    for line_res in reader.lines() {
        let rgb = line_res
            .unwrap()
            .split(";")
            .map(|subgame| {
                let mut rgb = (0, 0, 0);

                for cap in re.captures_iter(&subgame) {
                    let value: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                    let name = cap.get(2).unwrap().as_str();

                    match name {
                        "red" => rgb.0 = value,
                        "green" => rgb.1 = value,
                        "blue" => rgb.2 = value,
                        _default => panic!(),
                    };
                }
                rgb
            })
            .reduce(|acc, cur| (max(acc.0, cur.0), max(acc.1, cur.1), max(acc.2, cur.2)))
            .unwrap();

        sum += rgb.0 * rgb.1 * rgb.2;
    }
}
