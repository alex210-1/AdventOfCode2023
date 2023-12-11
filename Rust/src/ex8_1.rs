use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node([char; 3]);

impl Node {
    fn from_str(str: &str) -> Self {
        let arr: [char; 3] = str.chars().collect_vec().try_into().unwrap();
        Node(arr)
    }

    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }

    fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

pub fn run() {
    let re_node = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

    let input = read_to_string("./ex8-1.txt").unwrap();
    let sections = input.split("\n\n").collect_vec();

    let dir_iter = sections[0].chars().cycle();

    let mut map: HashMap<Node, (Node, Node)> = HashMap::new();
    let mut start_nodes: Vec<Node> = Vec::new();

    for cap in re_node.captures_iter(sections[1]) {
        let name = Node::from_str(cap.get(1).unwrap().as_str());
        let left = Node::from_str(cap.get(2).unwrap().as_str());
        let right = Node::from_str(cap.get(3).unwrap().as_str());

        map.insert(name, (left, right));

        if name.is_start() {
            start_nodes.push(name);
        }
    }

    let steps = start_nodes
        .iter()
        .map(|node| {
            let mut steps = 0u128;
            let mut cur_node = node;

            let mut cur_dir_iter = dir_iter.clone();

            loop {
                let dir = cur_dir_iter.next().unwrap();
                let paths = map.get(cur_node).unwrap();

                cur_node = match dir {
                    'L' => &paths.0,
                    'R' => &paths.1,
                    _ => panic!(),
                };

                steps += 1;
                if cur_node.is_end() {
                    break;
                }
            }

            println!("steps: {steps}");
            steps
        })
        .collect_vec();

    let res = steps.into_iter().reduce(|a, b| lcm(a, b)).unwrap();

    println!("gcd: {res}");
}
