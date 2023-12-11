use std::cmp::{PartialEq, Eq, Ord, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

use crate::util::MyResult;
use crate::util::return_part1and2;
use crate::util::RunResult;

use regex::Regex;

// Using an enum would be more "rust-y" but god that's a ton of boilerplate
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Card(char);

impl Card {
    fn value(&self) -> Option<u32> {
        if '1' <= self.0 && self.0 <= '9' {
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
    fn compute_kind(&self) -> HandKind {
        let mut counts: Vec<(u32, Card)> = Vec::new();
        let mut joker_count = 0;
        for c in self.cards {
            let mut found = false;
            if c.0 == '1' {
                joker_count += 1;
                continue;
            }
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
        if counts.len() == 0 {
            // only possible for a hand containing all jokers... create a
            // zero-count best card
            counts.push((0, Card('A')))
        }

        counts.sort_by(|a, b| b.cmp(a));
        counts[0].0 += joker_count;

        if counts[0].0 == 5 {
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
        }
    }

    fn new(s: &str, expr: &Regex) -> MyResult<Hand> {
        let c = expr.captures(s).ok_or("invalid hand input")?;
        let bidstr = c.get(2).unwrap().as_str();
        let bid = isize::from_str_radix(bidstr, 10)?;
        let cardstr = c.get(1).unwrap().as_str();
        let cards: [Card; 5] = cardstr.chars()
                                      .map(|c| Card(c))
                                      .collect::<Vec<Card>>()
                                      .try_into().unwrap();

        let mut hand = Hand{cards, kind: HandKind::HighCard, bid};
        hand.kind = hand.compute_kind();
        Ok(hand)
    }

    fn convert_jokers(&mut self) {
        for c in self.cards.iter_mut() {
            if c.0 == 'J' {
                c.0 = '1'
            }
        }
    }
}

fn score(hands: &Vec<Hand>) -> isize {
    let mut value: isize = 0;
    for (i, hand) in hands.iter().enumerate() {
        value += (i as isize + 1) * hand.bid;
    }
    value
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
    let part1 = score(&hands);
    println!("Part 1: {}", part1);

    for hand in hands.iter_mut() {
        hand.convert_jokers();
        hand.kind = hand.compute_kind();
    }
    hands.sort();
    let part2 = score(&hands);
    println!("Part 2: {}", part2);
    return_part1and2(part1, part2)
}
