use std::cmp::{PartialEq, Eq, Ord, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

use crate::util::MyResult;
use crate::util::return_part1;
use crate::util::RunResult;

use regex::Regex;

// Using an enum would be more "rust-y" but god that's a ton of boilerplate
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Card(char);

impl Card {
    fn value(&self) -> Option<u32> {
        if '2' <= self.0 && self.0 <= '9' {
            self.0.to_digit(10)
        } else {
            match self.0 {
                'T' => Some(10),
                'J' => Some(11),
                'Q' => Some(12),
                'K' => Some(13),
                'A' => Some(14),
                _ => None
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().unwrap().cmp(&other.value().unwrap())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandKind,
    cards: [Card; 5],
    bid: isize,
}

impl Hand {
    fn new(s: &str, expr: &Regex) -> MyResult<Hand> {
        let c = expr.captures(s).ok_or("invalid hand input")?;
        let bidstr = c.get(2).unwrap().as_str();
        let bid = isize::from_str_radix(bidstr, 10)?;
        let cardstr = c.get(1).unwrap().as_str();
        let cards: [Card; 5] = cardstr.chars()
                                      .map(|c| Card(c))
                                      .collect::<Vec<Card>>()
                                      .try_into().unwrap();

        let mut counts: Vec<(u32, Card)> = Vec::new();
        for c in cards {
            let mut found = false;
            for i in 0..counts.len() {
                if c == counts[i].1 {
                    counts[i].0 += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                counts.push((1, c));
            }
        }
        counts.sort_by(|a, b| b.cmp(a));
        let kind = if counts[0].0 == 5 {
            HandKind::FiveOfAKind
        } else if counts[0].0 == 4 {
            HandKind::FourOfAKind
        } else if counts[0].0 == 3 && counts[1].0 == 2 {
            HandKind::FullHouse
        } else if counts[0].0 == 3 {
            HandKind::ThreeOfAKind
        } else if counts[0].0 == 2 && counts[1].0 == 2 {
            HandKind::TwoPair
        } else if counts[0].0 == 2 {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        };
        Ok(Hand{cards, kind, bid})
    }
}

pub fn run(fln: &str) -> RunResult {
    let r = BufReader::new(File::open(fln)?);
    let expr = Regex::new(r"([AKQJT2-9]{5}) (\d+)")?;
    let mut hands: Vec<Hand> = Vec::new();
    for line in r.lines() {
        let line = line?;
        hands.push(Hand::new(&line, &expr)?);
    }
    hands.sort();
    let mut value: isize = 0;
    for (i, hand) in hands.iter().enumerate() {
        value += (i as isize + 1) * hand.bid;
    }
    println!("Part 1: {}", value);
    return_part1(value)
}
