use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

const SIZE: usize = 140;

struct Galaxy {
    x: usize,
    y: usize,
}

pub fn run() {
    let input = read_to_string("./ex11-1.txt").unwrap();

    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut filled_rows: HashSet<usize> = HashSet::new();
    let mut filled_cols: HashSet<usize> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy { x, y });

                filled_rows.insert(y);
                filled_cols.insert(x);
            }
        }
    }

    let full_set: HashSet<usize> = (0..SIZE).into_iter().collect();
    let empty_rows = full_set
        .difference(&filled_rows)
        .sorted()
        .rev()
        .collect_vec();
    let empty_cols = full_set
        .difference(&filled_cols)
        .sorted()
        .rev()
        .collect_vec();

    for empty_row in empty_rows {
        for galaxy in &mut galaxies {
            if galaxy.y > *empty_row {
                galaxy.y += 1000000 - 1;
            }
        }
    }
    for empty_col in empty_cols {
        for galaxy in &mut galaxies {
            if galaxy.x > *empty_col {
                galaxy.x += 1000000 - 1;
            }
        }
    }

    let sum: usize = galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
        .sum();

    println!("sum: {sum}");
}
