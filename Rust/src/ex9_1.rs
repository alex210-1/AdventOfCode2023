use std::fs::read_to_string;

use itertools::Itertools;

fn solve_line(line: &Vec<i64>) -> i64 {
    println!("{:?}", line);

    if line.iter().all(|v| *v == 0) {
        0
    } else {
        let line_below = line.iter().map_windows(|[a, b]| *b - *a).collect_vec();
        let solution_below = solve_line(&line_below);

        let first = line.first().unwrap();

        first - solution_below
    }
}

pub fn run() {
    let input = read_to_string("./ex9-1.txt").unwrap();

    let sum: i64 = input
        .lines()
        .map(|line| {
            let vals: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
            let sol = solve_line(&vals);

            println!("sol: {sol}");
            sol
        })
        .sum();

    println!("sum: {sum}");
}
