use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter;
use std::result::Result;
use std::vec::Vec;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

enum Dir {
    Left,
    Right,
}

fn read_input(filename: &str) -> MyResult<Vec<Dir>> {
    let mut input = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut input)?;
    let mut v: Vec<Dir> = Vec::new();
    use Dir::*;
    for c in input.trim().chars() {
        match c {
            '>' => v.push(Right),
            '<' => v.push(Left),
            _ => return Err("invalid jet direction character".into())
        }
    }
    Ok(v)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Rock,
}

struct Cave {
    width: isize,
    cave: Vec<Space>,
}

impl Cave {
    fn new(width: isize) -> Self {
        Self{width, cave: Vec::new()}
    }
    fn get(&self, c: &Coord) -> Space {
        let index = (c.y * self.width + c.x) as usize;
        if index < self.cave.len() {
            self.cave[index]
        } else {
            Space::Empty
        }
    }
    fn set(&mut self, c: &Coord, sp: Space) {
        let index = (c.y * self.width + c.x) as usize;
        while index >= self.cave.len() {
            self.cave.extend(iter::repeat(Space::Empty).take(10 * self.width as usize))
        }
        self.cave[index] = sp;
    }
    fn top_rock(&self) -> Option<Coord> {
        for i in (0..self.cave.len()).rev() {
            if let Space::Rock = self.cave[i] {
                return Some(Coord{x: i as isize % self.width, y: i as isize / self.width})
            }
        }
        None
    }
    #[allow(dead_code)]  // For debugging
    fn print(&self) {
        for y in (0 ..= self.cave.len() as isize / self.width).rev() {
            for x in 0 .. self.width {
                match self.get(&Coord{x, y}) {
                    Space::Rock => print!("#"),
                    Space::Empty => print!(" "),
                }
            }
            print!("\n");
        }
    }
}

const SHAPES: [&[Coord]; 5] = [
    /* line: ---- */
    &[Coord{x: 0, y: 0}, Coord{x: 1, y: 0}, Coord{x: 2, y: 0}, Coord{x: 3, y: 0}],
    /* plus */
    &[Coord{x: 0, y: 1}, Coord{x: 1, y: 0}, Coord{x: 1, y: 1}, Coord{x: 1, y: 2}, Coord{x: 2, y: 1}],
    /* L */
    &[Coord{x: 0, y: 0}, Coord{x: 1, y: 0}, Coord{x: 2, y: 0}, Coord{x: 2, y: 1}, Coord{x: 2, y: 2}],
    /* Line: | */
    &[Coord{x: 0, y: 0}, Coord{x: 0, y: 1}, Coord{x: 0, y: 2}, Coord{x: 0, y: 3}],
    /* Square */
    &[Coord{x: 0, y: 0}, Coord{x: 1, y: 0}, Coord{x: 0, y: 1}, Coord{x: 1, y: 1}],
];

fn add(cave: &Cave, coords: &mut Vec<Coord>, x: isize, y: isize) -> Result<(), ()> {
    /* First check */
    for c in coords.iter() {
        let new = Coord{x: c.x + x, y: c.y + y};
        if new.x < 0 || new.x >= cave.width || new.y < 0 {
            return Err(())
        }
        if let Space::Rock = cave.get(&new) {
            return Err(())
        }
    }
    /* Then execute */
    for c in coords.iter_mut() {
        c.x += x;
        c.y += y;
    }
    Ok(())
}

fn fall_until(jets: &Vec<Dir>, until: usize) -> usize {
    let mut fallen: usize = 0;
    let mut falling: Vec<Coord>= Vec::from(SHAPES[0]);
    let mut shape_index = 1;
    let mut jet_index = 0;
    let mut cave = Cave::new(7);

    let cycle_len = jets.len() * SHAPES.len();
    let mut cycle: Vec<(isize, usize)> = Vec::new();
    let mut prev_cycle: Option<Vec<(isize, usize)>> = None;
    let mut top = 0;
    let mut prev_top = 0;
    let mut prev_fallen = 0;

    /* To start, position the shape 3 blocks above and 2 right */
    add(&cave, &mut falling, 2, 3).ok();

    while fallen < until {
        /* First, do the jet of air. If the move is impossible, continue */
        match jets[jet_index] {
            Dir::Left => add(&cave, &mut falling, -1, 0).ok(),
            Dir::Right => add(&cave, &mut falling, 1, 0).ok(),
        };

        cycle.push((top - prev_top, fallen - prev_fallen));
        if cycle.len() == cycle_len {
            match prev_cycle {
                Some(prev_vec) if prev_vec == cycle => {
                    println!("Found pattern!");
                    let fallen_per_cycle = fallen - prev_fallen;
                    let blocks_per_cycle = top - prev_top;
                    let full_cycles = (until - fallen) / fallen_per_cycle;
                    let fallen_target = (until - fallen) % fallen_per_cycle;
                    let blocks_full_cycle = top + blocks_per_cycle * full_cycles as isize;
                    for i in 0..cycle_len {
                        if cycle[i].1 == fallen_target {
                            return (blocks_full_cycle + cycle[i].0) as usize + 1;
                        }
                    }
                    panic!("Should have found an answer.");
                },
                _ => {
                    println!("Ran for cycle with no pattern");
                    prev_cycle = Some(cycle);
                    cycle = Vec::with_capacity(cycle_len);
                    prev_top = top;
                    prev_fallen = fallen;
                }
            }
        }
        jet_index = (jet_index + 1) % jets.len();

        /* Next, move the block down. If the move is impossible, generate new block */
        if let Err(()) = add(&cave, &mut falling, 0, -1) {
            for c in falling.iter() {
                cave.set(c, Space::Rock);
            }
            /* Select new shape as falling block */
            falling.clear();
            falling.extend(SHAPES[shape_index]);
            shape_index = (shape_index + 1) % SHAPES.len();
            add(&cave, &mut falling, 2, cave.top_rock().unwrap().y + 4).unwrap();
            fallen += 1;
            top = cave.top_rock().unwrap().y;
        }
    }

    cave.top_rock().unwrap().y as usize + 1
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let jets = read_input(filename).unwrap();
    let height = fall_until(&jets, 2022);
    println!("After block 2022, block height is {}", height);

    println!("The input length is {}, and the shape cycle length is {}", jets.len(), SHAPES.len());
    let height = fall_until(&jets, 1000000000000);
    println!("After block 1 trillion, block height is {}", height);
}
