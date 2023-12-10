#![feature(iter_array_chunks)]

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::fs::read_to_string;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Clone, Copy)]
struct PlantRange {
    from: i64,
    to: i64,
}

struct PlantRule {
    pub destination: i64,
    pub source: i64,
    pub length: i64,
}
impl PlantRule {
    // apply rule to a range. return new source ranges and target range
    fn cut(&self, range: PlantRange) -> (Vec<PlantRange>, Option<PlantRange>) {
        // println!(
        //     "rule s: {}, d: {}, l: {}",
        //     self.source, self.destination, self.length
        // );

        if range.from < self.source + self.length && self.source <= range.to {
            // intersection
            let mut sources = Vec::new();

            // split source
            if self.source > range.from {
                sources.push(PlantRange {
                    from: range.from,
                    to: self.source - 1,
                });
            }
            if self.source + self.length - 1 < range.to {
                sources.push(PlantRange {
                    from: self.source + self.length,
                    to: range.to,
                })
            }

            let offset = self.destination - self.source;
            let target = PlantRange {
                from: offset + max(self.source, range.from),
                to: offset + min(self.source + self.length - 1, range.to), // TODO check offbyone
            };

            // println!(
            //     "target (orig): {}-{} [{}]",
            //     target.from - offset,
            //     target.to - offset,
            //     target.to - target.from + 1
            // );
            // println!("target (new):  {}-{}", target.from, target.to);
            // for source in sources.clone() {
            //     println!("source: {}-{}", source.from, source.to);
            // }
            // println!();

            (sources, Some(target))
        } else {
            // println!("no match\n");
            (vec![range], None)
        }
    }
}

struct PlantMap {
    rules: Vec<PlantRule>,
}
impl PlantMap {
    fn parse(source: &str) -> Self {
        let mut lines = source.lines();
        lines.next(); // ignore header

        let rules = lines
            .map(|line| {
                let vals: Vec<i64> = RE
                    .find_iter(line)
                    .map(|s| s.as_str().parse().unwrap())
                    .collect();

                PlantRule {
                    destination: vals[0],
                    source: vals[1],
                    length: vals[2],
                }
            })
            .collect_vec();

        PlantMap { rules }
    }

    // fn map(&self, val: i64) -> i64 {
    //     for range in self.ranges.iter() {
    //         if val >= range.source && val < range.source + range.length {
    //             return range.destination + val - range.source;
    //         }
    //     }
    //     return val;
    // }

    fn map_range(&self, val: PlantRange) -> Vec<PlantRange> {
        // println!("==> map: range {}-{}", val.from, val.to);

        let mut source_ranges = vec![val];
        let mut target_ranges: Vec<PlantRange> = Vec::new();

        for rule in &self.rules {
            source_ranges = source_ranges
                .iter()
                .flat_map(|range| {
                    let (new_sources, target_op) = rule.cut(*range);

                    if let Some(target) = target_op {
                        target_ranges.push(target);
                    }

                    new_sources
                })
                .collect();
        }

        target_ranges.append(&mut source_ranges);
        target_ranges
    }
}

pub fn run() {
    let input = read_to_string("./ex5-1.txt").unwrap();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let sections = input.split("\n\n").collect_vec();

    let seeds: Vec<PlantRange> = RE
        .find_iter(sections[0])
        .map(|m| m.as_str().parse().unwrap())
        .array_chunks::<2>()
        .map(|[from, length]| PlantRange {
            from,
            to: from + length - 1,
        })
        .collect();

    let maps = sections[1..]
        .iter()
        .map(|source| PlantMap::parse(&source))
        .collect_vec();

    let locations = maps.iter().fold(seeds, |sources: Vec<PlantRange>, map| {
        // println!("===> level");
        // for source in sources.clone() {
        //     println!("\tsource: {}-{}", source.from, source.to);
        // }

        sources
            .iter()
            .flat_map(|range| map.map_range(*range))
            .collect()
    });

    let location = locations.iter().map(|range| range.from).min().unwrap();

    // let location: i64 = seeds
    //     .iter()
    //     .map(|seed_range| maps.iter().fold(*seed, |val, map| map.map(val)))
    //     .min()
    //     .unwrap();

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("\nmin: {}", location);
    println!("Time: {}", (end - start).as_micros())
}
