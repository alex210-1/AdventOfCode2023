use std::cmp::{Ord, Ordering};
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
struct Card(u8);

impl Card {
    fn from_char(c: char) -> Self {
        let value = match c {
            'J' => 1,
            m @ '2'..='9' => m.to_digit(10).unwrap(),
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!(),
        };
        Card(value as u8)
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5], // sorted  high to low
    rank: Rank,
    bid: u64,
}
impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Self {
        let rank = Self::get_rank_jokered(cards.clone());

        Hand { cards, rank, bid }
    }

    fn get_rank_jokered(cards: [Card; 5]) -> Rank {
        let joker_pos = cards
            .iter()
            .enumerate()
            .filter(|(_, card)| card.0 == 1)
            .map(|(i, _)| i)
            .collect_vec();

        if joker_pos.is_empty() {
            Self::get_rank(cards)
        } else {
            // Joker
            (2..=13)
                .map(|substitute| {
                    let mut sub_cards = cards.clone();

                    for joker in &joker_pos {
                        sub_cards[*joker] = Card(substitute);
                    }
                    Self::get_rank(sub_cards)
                })
                .max_by_key(|&r| r as u8)
                .unwrap()
        }
    }

    fn get_rank(cards: [Card; 5]) -> Rank {
        let groups = cards
            .iter()
            .sorted()
            .rev()
            .peekable()
            .batching(|it| match it.next() {
                None => None,
                Some(card) => {
                    let mut same = 1;
                    loop {
                        match it.peek() {
                            Some(&next) if next == card => {
                                it.next();
                                same += 1;
                            }
                            _ => break,
                        };
                    }
                    Some(same)
                }
            })
            .sorted()
            .rev()
            .collect_vec();

        match groups.len() {
            1 => Rank::FiveSame,
            2 => match groups[0] {
                4 => Rank::FourSame,
                3 => Rank::FullHouse,
                _ => panic!(),
            },
            3 => match groups[0] {
                3 => Rank::ThreeSame,
                2 => Rank::TwoPair,
                _ => panic!(),
            },
            4 => Rank::OnePair,
            5 => Rank::Distinct,
            _ => panic!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.rank as u8).cmp(&(other.rank as u8)) {
            Ordering::Equal => {}
            ord => return ord,
        };

        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Rank {
    FiveSame = 7,
    FourSame = 6,
    FullHouse = 5,
    ThreeSame = 4,
    TwoPair = 3,
    OnePair = 2,
    Distinct = 1,
}

fn main() {
    let re = Regex::new(r"([2-9TJQKA]+) (\d+)").unwrap();

    let in_file = File::open("./ex7-1.txt").unwrap();
    let reader = BufReader::new(in_file);

    let hands = reader.lines().map(|line| {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();

        let cards: [Card; 5] = caps
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .map(|c| Card::from_char(c))
            .collect_vec()
            .try_into()
            .unwrap();

        let bid: u64 = caps.get(2).unwrap().as_str().parse().unwrap();

        Hand::new(cards, bid)
    });

    let sum: u64 = hands
        .sorted()
        .enumerate()
        .map(|x| {
            println!("index: {}", x.0);
            x
        })
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum();

    println!("sum: {sum}");
}
