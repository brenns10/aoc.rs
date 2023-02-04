use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter;
use std::ops::{Add, Sub};
use std::result::Result;

use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct C2D(isize, isize);

impl Add for C2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for C2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl C2D {
    fn multiply(&self, val: isize) -> C2D {
        C2D(self.0 * val, self.1 * val)
    }
}

const FACINGS: &[C2D] = &[
    C2D(1, 0),  // right
    C2D(0, 1),  // down
    C2D(-1, 0), // left
    C2D(0, -1), // up
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Offmap,
    Open,
    Wall,
}

struct Map {
    arr: Vec<Cell>,
    width: usize,
    height: usize,
    start: C2D,
    edgesize: isize,
}

/*
 * CUBE GEOMETRY. Like, fuck me, this is hard. But here we go.
 *
 *            9
 *    +----------------+
 *    |\8            5/|
 *    | +------------+ |
 *    | |     1      | |
 * 12 | |4          2| | 10
 *    | |     3      | |
 *    | +------------+ |
 *    |/7            6\|
 *    +----------------+
 *            11
 *
 * Ok, so imagine that the inner square is the surface of a cube, and it is
 * sitting on a piece of paper. The outer square is the surface of the cube
 * facing you.
 *
 * Now imagine rolling this cube upward (north). The side which was on the paper
 * is now on the downward side (south). Or more importantly, edge #1 has become
 * edge #3, and so on. Rolling the cube downward (south) produces the opposite
 * translation of edges. Rolling the cube left and right similarly produces a
 * set of complementary edge transitions.
 *
 * Now also imagine that we measure an "offset" of your location: the offset is
 * measured in distance from the top (for vertical edges), distance from the
 * leftmost side (for horizontal edges), or distance from the side closest from
 * the paper (for edges on the axis we that are looking down). Some transitions
 * will require that the offset be "negated" to conform to the new perspective.
 *
 * Below, we have two sets of translations. The first is for up/down and the
 * second is for right/left. The translations are represented as tuples:
 * (original edge, new edge, needs_negation)
 *
 * With these translations, we can essentially move along the unfolded cube, and
 * keep track of where we are on it. Once we are back on a edge which is on the
 * paper (i.e. edge 1-4), we can use our offset information to compute our new
 * coordinate. This is how we're going to continue across cube edges.
 *
 * And can I just say? Wow, this is hard. I'm kicking myself because I'm
 * absolutely certain that there's some sort of simple, clever rule that could
 * do all of this in just a few lines of code, but here I am writing a 50 line
 * comment to describe several arrays of data used in some real shitty code. But
 * you know what? It's the last star I need to complete this challenge, and
 * goddamn it, I will complete this challenge.
 */
const UP_DOWN_TRANS: [(u8, u8, bool); 12] = [
    (1, 3, false),
    (3, 11, false),
    (11, 9, false),
    (9, 1, false),
    (6, 10, true),
    (10, 5, false),
    (5, 2, true),
    (2, 6, false),
    (7, 12, true),
    (12, 8, false),
    (8, 4, true),
    (4, 7, false),
];

const RIGHT_LEFT_TRANS: [(u8, u8, bool); 12] = [
    (2, 4, false),
    (4, 12, false),
    (12, 10, false),
    (10, 2, false),
    (1, 8, true),
    (8, 9, false),
    (9, 5, true),
    (5, 1, false),
    (3, 7, true),
    (7, 11, false),
    (11, 6, true),
    (6, 3, false),
];

fn facing_to_edge(facing: usize) -> u8 {
    match facing {
        0 => 2, /* facing right: edge 2 */
        1 => 3, /* facing down: edge 3 */
        2 => 4, /* facing left: edge 4 */
        3 => 1, /* facing up, edge 1 */
        _ => panic!("Bad facing value {}", facing)
    }
}

fn edge_to_facing(edge: u8) -> usize {
    match edge {
        2 => 2,
        3 => 3,
        4 => 0,
        1 => 1,
        _ => panic!("Bad edge value {}", edge)
    }
}

fn edge_offset(edge: u8, coord: C2D, edgesize: isize) -> isize {
    /* Only valid for edges 1-4, convert the coordinate to an offset given an
     * edge. */
    if edge % 2 == 1 {
        // top or bottom: use X coordinate % EDGSIZE
        coord.0 % edgesize
    } else {
        coord.1 % edgesize
    }
}

fn edge_transition(edge: u8, roll: usize) -> (u8, bool) {
    let reg = !(roll == 1 || roll == 2);
    let arr = if roll % 2 == 1 { UP_DOWN_TRANS } else { RIGHT_LEFT_TRANS };

    for tup in arr {
        if reg && tup.0 == edge {
            return (tup.1, tup.2)
        } else if !reg && tup.1 == edge {
            return (tup.0, tup.2)
        }
    }
    panic!("Bad edge in transition")
}

fn find_next(map: &Map, coord: C2D, facing: usize, edgesize: isize) -> (C2D, usize) {
    let edge = facing_to_edge(facing);
    let offset = edge_offset(edge, coord, edgesize);
    let mut explore: Vec<(C2D, u8, isize)> = Vec::new();
    let mut seen: HashSet<C2D> = HashSet::new();
    explore.push((coord, edge, offset));

    while !explore.is_empty() {
        let (this_coord, edge, offset) = explore.pop().unwrap();
        seen.insert(this_coord);
        //println!("Visiting coord: {:?}", this_coord);
        for dir in 0..FACINGS.len() {
            let new_coord = this_coord + FACINGS[dir].multiply(edgesize);
            //println!("  Consider block of coordinate {:?} (block: {}, {}), a move in direction {}", new_coord, new_coord.0 / EDGSIZE, new_coord.1 / EDGSIZE, dir);
            if !map.in_bounds(&new_coord) {
                //println!("  => out of bounds");
                continue;
            }
            if let Cell::Offmap = map.get(&new_coord) {
                // Not on the map, keep looking
                //println!("  => off map");
                continue;
            }
            if new_coord == coord {
                //println!("  => visiting original");
                continue;
            }
            // This is on the cube map, but would the transition leave us on an
            // edge which is in contact with the "paper"?
            let (new_edge, negate) = edge_transition(edge, dir);
            let new_offset = if negate { edgesize - 1 - offset } else { offset };
            //println!("    Transition from edge {} to {}, offset {} to {}", edge, new_edge, offset, new_offset);
            if new_edge <= 4 {
                // Yay, we found the new square, we just need to convert back to
                // a coordinate and facing.
                let b = C2D(new_coord.0 - new_coord.0 % edgesize, new_coord.1 - new_coord.1 % edgesize);
                let new_facing = edge_to_facing(new_edge);
                //println!("  {:?}", b);
                let real_coord = match new_edge {
                    1 => C2D(b.0 + new_offset, b.1),
                    2 => C2D(b.0 + edgesize - 1, b.1 + new_offset),
                    3 => C2D(b.0 + new_offset, b.1 + edgesize - 1),
                    4 => C2D(b.0, b.1 + new_offset),
                    _ => panic!("Bad edge"),
                };
                //println!("  Next: {:?} facing {} -> {:?} facing {}", coord, facing, real_coord, new_facing);
                return (real_coord, new_facing)
            }
            // Ok, this isn't the right location, keep searching
            //println!("    Not on the bottom edge, continuing");
            if !seen.contains(&new_coord) {
                //println!("    => pushed");
                explore.push((new_coord, new_edge, new_offset));
            }
        }
    }
    panic!("Colud not find next coordinate!")
}

impl Map {
    fn ix(&self, coord: &C2D) -> usize {
        if coord.0 < 0 || coord.1 < 0 {
            panic!("Negative coordinate");
        }
        coord.0 as usize + coord.1 as usize * self.width
    }
    fn read_string(s: &str) -> MyResult<Map> {
        let lines: Vec<_> = s.lines().collect();
        let width = lines.iter().map(|s| s.len()).max().unwrap();
        let height = lines.len();
        let arr: Vec<Cell> = iter::repeat(Cell::Offmap).take(width * height).collect();
        let mut map = Map{arr, width, height, start: C2D(0, 0), edgesize: 0};
        let mut count_spaces = 0;
        let mut first = true;
        for (y, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                let cell = match c {
                    ' ' => continue,
                    '.' => Cell::Open,
                    '#' => Cell::Wall,
                    _ => return Err("invalid char".into()),
                };
                count_spaces += 1;
                let coord = C2D(i as isize, y as isize);
                if first && cell == Cell::Open {
                    map.start = coord;
                    first = false;
                }
                let ix = map.ix(&coord);
                map.arr[ix] = cell;
            }
        }
        /* We now need to determine this cube's edge size. */
        let edge_size = ((count_spaces / 6) as f64).sqrt() as isize;
        if edge_size * edge_size * 6 != count_spaces {
            return Err("This map is not a cube!".into())
        }
        map.edgesize = edge_size;
        Ok(map)
    }
    fn get(&self, coord: &C2D) -> Cell {
        let ix = self.ix(coord);
        self.arr[ix]
    }
    fn in_bounds(&self, coord: &C2D) -> bool {
        (0 <= coord.0) && (coord.0 < self.width as isize) && (0 <= coord.1) && (coord.1 < self.height as isize)
    }
    fn mov(&self, coord: C2D, dir: usize, count: usize) -> (C2D, usize) {
        let mut cur = coord;
        let dirc = FACINGS[dir];
        for _ in 0..count {
            let next = cur + dirc;
            if !self.in_bounds(&next) || self.get(&next) == Cell::Offmap {
                let mut bt = cur;
                while self.in_bounds(&(bt - dirc)) && self.get(&(bt - dirc)) != Cell::Offmap {
                    bt = bt - dirc;
                }
                if self.get(&bt) == Cell::Open {
                    cur = bt;
                }
            } else if self.get(&next) == Cell::Open {
                cur = next;
            } else {
                break;
            }
        }
        (cur, dir)
    }
    fn mov_cube(&self, coord: C2D, dir: usize, count: usize) -> (C2D, usize) {
        let mut dir = dir;
        let mut cur = coord;
        let mut dirc = FACINGS[dir];
        for _ in 0..count {
            let next = cur + dirc;
            if !self.in_bounds(&next) || self.get(&next) == Cell::Offmap {
                //println!("At {:?} facing {}, next {:?} is out of bounds or off map",
                //    cur, dir, next);
                let (bt, facing) = find_next(self, cur, dir, self.edgesize);
                if self.get(&bt) == Cell::Open {
                    cur = bt;
                    dir = facing;
                    dirc = FACINGS[dir];
                } else {
                    break; /* welp we've hit a wall */
                }
            } else if self.get(&next) == Cell::Open {
                cur = next;
            } else {
                break;
            }
        }
        (cur, dir)
    }
    fn print(&self, pos: &C2D) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cur = C2D(x as isize, y as isize);
                let cell = self.get(&cur);
                if cur == *pos {
                    if cell != Cell::Open {
                        panic!("Bad location!")
                    }
                    print!("@");
                    continue;
                }
                let s = match cell {
                    Cell::Offmap => " ",
                    Cell::Open => ".",
                    Cell::Wall => "#",
                };
                print!("{}", s);
            }
            print!("\n");
        }
    }
}

enum Instruction {
    Left,
    Right,
    Move(usize),
}

fn read_input(filename: &str) -> MyResult<(Map, Vec<Instruction>)> {
    let mut reader = File::open(filename)?;
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let (map, inst) = contents.split_once("\n\n").ok_or_else(|| "Bad map separation")?;
    let mapval = Map::read_string(map)?;
    let expr = Regex::new(r"(\d+)|L|R")?;
    let mut instrs = Vec::new();
    for v in expr.find_iter(inst) {
        if v.as_str() == "L" {
            instrs.push(Instruction::Left);
        } else if v.as_str() == "R" {
            instrs.push(Instruction::Right);
        } else {
            instrs.push(Instruction::Move(usize::from_str_radix(v.as_str(), 10)?));
        }
    }
    Ok((mapval, instrs))
}

fn do_path(map: &Map, instrs: &Vec<Instruction>, cube: bool, verbose: bool) {
    let mut facing: usize = 0;
    let mut coord = map.start;
    if verbose {
        println!("Start:");
        map.print(&coord);
    }
    for instr in instrs.iter() {
        use Instruction::*;
        match instr {
            Left => {
                facing = (facing + FACINGS.len() - 1) % FACINGS.len();
                if verbose {println!("Pivot left, new facing is: {}", facing)};
            },
            Right => {
                facing = (facing + 1) % FACINGS.len();
                if verbose {println!("Pivot right, new facing is: {}", facing)};
            },
            Move(amt) => {
                if verbose { println!("Moving {}...", amt) }
                if cube {
                    (coord, facing) = map.mov_cube(coord, facing, *amt)
                } else {
                    (coord, facing) = map.mov(coord, facing, *amt)
                }
                if verbose {map.print(&coord)}
            }
        }
    }
    println!("Final row={}, column={}, facing={}", coord.1, coord.0, facing);
    println!("Password: {}", (coord.1 + 1) * 1000 + (coord.0 + 1) * 4 + facing as isize);
}

fn main() {
    let mut filename = "input.txt";
    let mut verbose = false;
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
        verbose = true;
    }
    let (map, instrs) = read_input(filename).unwrap();
    do_path(&map, &instrs, false, false);
    do_path(&map, &instrs, true, verbose);
}
