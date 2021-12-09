use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;
use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error>>;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Navigation {
    direction: Direction,
    amount: i32,
}

fn read_navigation() -> BoxResult<Vec<Navigation>> {
    let mut res: Vec<Navigation> = Vec::new();
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("forward ") {
            res.push(Navigation{direction: Direction::Forward, amount: line[8..].parse()?});
        } else if line.starts_with("up ") {
            res.push(Navigation{direction: Direction::Up, amount: line[3..].parse()?});
        } else {
            res.push(Navigation{direction: Direction::Down, amount: line[5..].parse()?});
        }
    }
    Ok(res)
}

struct Position {
    depth: i32,
    horiz: i32,
    aim: i32,
}

fn do_navigation(instrs: &Vec<Navigation>) -> Position {
    let mut pos = Position{depth: 0, horiz: 0, aim: 0};

    for instr in instrs {
        match instr.direction {
            Direction::Up => {pos.depth -= instr.amount},
            Direction::Down => {pos.depth += instr.amount},
            Direction::Forward => {pos.horiz += instr.amount},
        }
    }

    pos
}

fn do_nav_with_aim(instrs: &Vec<Navigation>) -> Position {
    let mut pos = Position{depth: 0, horiz: 0, aim: 0};

    for instr in instrs {
        match instr.direction {
            Direction::Up => {pos.aim -= instr.amount},
            Direction::Down => {pos.aim += instr.amount},
            Direction::Forward => {
                pos.horiz += instr.amount;
                pos.depth += pos.aim * instr.amount;
            },
        }
    }

    pos
}

fn main() {
    let instrs = read_navigation().unwrap();

    let nav_res = do_navigation(&instrs);
    println!("Final position: depth: {}, horiz: {}", nav_res.depth, nav_res.horiz);
    println!("  product: {}", nav_res.depth * nav_res.horiz);

    let nav_res = do_nav_with_aim(&instrs);
    println!("Final position: depth: {}, horiz: {}", nav_res.depth, nav_res.horiz);
    println!("  product: {}", nav_res.depth * nav_res.horiz);
}
