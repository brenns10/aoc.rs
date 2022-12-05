use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;
use std::vec::Vec;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(PartialEq, Eq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for RPS {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err("Invalid RPS code"),
        }
    }
}

enum RPSResult {
    Win,
    Lose,
    Draw
}

impl TryFrom<&str> for RPSResult {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid RPS result code"),
        }
    }
}

fn rps_outcome(mine: RPS, theirs: RPS) -> RPSResult {
    match (mine, theirs) {
        (a, b) if a == b => RPSResult::Draw,
        (RPS::Paper, RPS::Rock) => RPSResult::Win,
        (RPS::Rock, RPS::Scissors) => RPSResult::Win,
        (RPS::Scissors, RPS::Paper) => RPSResult::Win,
        _ => RPSResult::Lose,
    }
}

fn rps_score(mine: RPS, theirs: RPS) -> u32 {
    let outcome = rps_outcome(mine, theirs);
    let play_score = match mine {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    let outcome_score = match outcome {
        RPSResult::Win => 6,
        RPSResult::Draw => 3,
        RPSResult::Lose => 0,
    };
    play_score + outcome_score
}

fn read_guide() -> MyResult<Vec<(RPS, RPS)>> {
    let mut res: Vec<(RPS, RPS)> = Vec::new();
    for line in io::BufReader::new(File::open("input.txt")?).lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(" ").collect();
        assert_eq!(moves.len(), 2);
        let theirs = RPS::try_from(moves[0])?;
        let mine = RPS::try_from(moves[1])?;
        res.push((mine, theirs))
    }
    Ok(res)
}

fn rps_pick(theirs: RPS, outcome: RPSResult) -> RPS {
    match (theirs, outcome) {
        (mv, RPSResult::Draw) => mv,
        (RPS::Paper, RPSResult::Win) => RPS::Scissors,
        (RPS::Scissors, RPSResult::Win) => RPS::Rock,
        (RPS::Rock, RPSResult::Win) => RPS::Paper,
        (RPS::Scissors, RPSResult:: Lose) => RPS::Paper,
        (RPS::Rock, RPSResult::Lose) => RPS::Scissors,
        (RPS::Paper, RPSResult::Lose) => RPS::Rock,
    }
}

fn read_guide_fixed() -> MyResult<Vec<(RPS, RPSResult)>> {
    let mut res: Vec<(RPS, RPSResult)> = Vec::new();
    for line in io::BufReader::new(File::open("input.txt")?).lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(" ").collect();
        assert_eq!(moves.len(), 2);
        let theirs = RPS::try_from(moves[0])?;
        let outcome = RPSResult::try_from(moves[1])?;
        res.push((theirs, outcome))
    }
    Ok(res)
}

fn main() {
    let guide = read_guide().unwrap();
    let total_score: u32 = guide.into_iter().map(|t| rps_score(t.0, t.1)).sum();
    println!("Total score (wrong): {}", total_score);
    let guide = read_guide_fixed().unwrap();
    let total_score: u32 = guide.into_iter().map(|t| rps_score(rps_pick(t.0, t.1), t.0)).sum();
    println!("Total score: {}", total_score);
}
