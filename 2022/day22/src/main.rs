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

/// C2D - coordinate in 2 dimensions. For this challenge I went with (X, Y)
/// notation here rather than (row, column), but also Y increases as you go
/// down. Don't ask me what I was thinking.
///
/// As is usual for my coordinates, they can be used for addition/subtraction,
/// but also I added a multiply function to help with stepping over great
/// distances.
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

/// FACINGS are the directions you could be facing while traversing the map.
/// This is directly from the problem description.
const FACINGS: &[C2D] = &[
    C2D(1, 0),  // right
    C2D(0, 1),  // down
    C2D(-1, 0), // left
    C2D(0, -1), // up
];

/// The content of a particular location of the map can either be a wall, an
/// open space, or something that's off the map but not out of bounds of the
/// array.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Offmap,
    Open,
    Wall,
}

#[derive(Clone, Copy)]
enum StepStrategy {
    Flat,
    Cube,
}

/// Map - this is some variation on my somewhat "standard" map structure, where
/// you have a small finite boundary indexed in two dimensions. It uses the X, Y
/// method of indexing, with Y increasing as you go down. The map also stores
/// the edge size of the cube - this is the main difference compared to other
/// versions. The map doesn't contain too much in the way of cube logic, the
/// idea is to keep that separate.
struct Map {
    arr: Vec<Cell>,
    width: usize,
    height: usize,
    start: C2D,
    edgesize: isize,
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
 * Now also imagine that we measure an "offset" which is the location along the
 * edge: the offset is measured in distance from the top (for vertical edges),
 * distance from the leftmost side (for horizontal edges), or distance from the
 * side closest from the paper (for edges on the axis we that are looking down).
 * Some transitions will require that the offset be "negated" to conform to the
 * new perspective.
 *
 * Below, we have two sets of translations. The first is for up/down and the
 * second is for right/left. The translations are represented as tuples:
 * (original edge, new edge, needs_negation). They are written in one direction
 * only, but the reverse direction is (new edge, original edge, needs_negation).
 * So they can be used in both situations.
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

/// Translate from a "facing" (which is an index into the FACINGS array and also
/// a number used in the problem description) into an edge number.
///
/// This is intended be used when we're leaving one face of the cube. The
/// direction we're facing corresponds to the edge we must currently be at.
fn facing_to_edge(facing: usize) -> u8 {
    match facing {
        0 => 2, /* facing right: edge 2 */
        1 => 3, /* facing down: edge 3 */
        2 => 4, /* facing left: edge 4 */
        3 => 1, /* facing up, edge 1 */
        _ => panic!("Bad facing value {}", facing)
    }
}

/// Translate from an edge number to the corresponding facing. As opposed to the
/// above function, this is actually a different mapping, because it is designed
/// to be used for the case where you are *entering* a new cube face. Entering a
/// cube face while facing right means that you are entering the left side, thus
/// the difference.
fn edge_to_facing(edge: u8) -> usize {
    match edge {
        2 => 2,
        3 => 3,
        4 => 0,
        1 => 1,
        _ => panic!("Bad edge value {}", edge)
    }
}

/// Given an edge on the bottom face of the cube, and a coordinate, return the
/// offset. The offset is what we'll actually keep track of as we roll the cube
/// around in search of the new coordinate.
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

/// Given an edge and a direction we're rolling the cube, return the new edge
/// number and a flag true if we need to negate the offset.
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

/// Step off the map, assuming it's a cube. This implements one possible "step
/// strategy". See the above CUBE GEOMETRY comment for a full description of
/// what edge numbers are, how offset is defined, and the transition
/// definitions.
///
/// The algorithm is basically a BFS. We determine which edge of the cube we're
/// on, and then "roll" the cube around until we find a new section of the map
/// where the same edge is also on the bottom. Then we make the necessary
/// translation back into a coordinate and facing direction.
fn step_cube(map: &Map, coord: C2D, facing: usize) -> (C2D, usize) {
    let edge = facing_to_edge(facing);
    let edgesize = map.edgesize;
    let offset = edge_offset(edge, coord, edgesize);
    let mut explore: Vec<(C2D, u8, isize)> = Vec::new();
    let mut seen: HashSet<C2D> = HashSet::new();
    explore.push((coord, edge, offset));

    while !explore.is_empty() {
        let (this_coord, edge, offset) = explore.pop().unwrap();
        seen.insert(this_coord);
        for dir in 0..FACINGS.len() {
            let new_coord = this_coord + FACINGS[dir].multiply(edgesize);
            if !map.in_bounds(&new_coord) {
                continue;
            }
            if let Cell::Offmap = map.get(&new_coord) {
                continue;
            }
            if new_coord == coord {
                continue;
            }
            // This is on the cube map, but would the transition leave us on an
            // edge which is in contact with the "paper"?
            let (new_edge, negate) = edge_transition(edge, dir);
            let new_offset = if negate { edgesize - 1 - offset } else { offset };
            if new_edge <= 4 {
                // Yay, we found the new square, we just need to convert back to
                // a coordinate and facing.
                let b = C2D(new_coord.0 - new_coord.0 % edgesize, new_coord.1 - new_coord.1 % edgesize);
                let new_facing = edge_to_facing(new_edge);
                let real_coord = match new_edge {
                    1 => C2D(b.0 + new_offset, b.1),
                    2 => C2D(b.0 + edgesize - 1, b.1 + new_offset),
                    3 => C2D(b.0 + new_offset, b.1 + edgesize - 1),
                    4 => C2D(b.0, b.1 + new_offset),
                    _ => panic!("Bad edge"),
                };
                return (real_coord, new_facing)
            }
            // Ok, this isn't the right location, keep searching
            //println!("    Not on the bottom edge, continuing");
            if !seen.contains(&new_coord) {
                explore.push((new_coord, new_edge, new_offset));
            }
        }
    }
    panic!("Could not find next coordinate!")
}

/// Step off the map, assuming it's flat, like pac-man. You just need to
/// backtrack to the other side of the map.
fn step_flat(map: &Map, coord: C2D, dir: usize) -> (C2D, usize) {
    let mut bt = coord;
    while map.in_bounds(&(bt - FACINGS[dir])) && map.get(&(bt - FACINGS[dir])) != Cell::Offmap {
        bt = bt - FACINGS[dir];
    }
    (bt, dir)
}

/// Move starting from coord in direction dir, for count steps, using the given
/// step strategy. The step strategy is used to figure out what happens when we
/// leave one side of the map.
/// Return the new location and the new direction.
fn do_move(map: &Map, coord: C2D, dir: usize, count: usize, strat: StepStrategy) -> (C2D, usize) {
    let mut dir = dir;
    let mut cur = coord;
    for _ in 0..count {
        let mut next = cur + FACINGS[dir];
        let mut next_dir = dir;
        if !map.in_bounds(&next) || map.get(&next) == Cell::Offmap {
            (next, next_dir) = match strat {
                StepStrategy::Flat => step_flat(map, cur, dir),
                StepStrategy::Cube => step_cube(map, cur, dir),
            };
        }
        if map.get(&next) == Cell::Open {
            cur = next;
            dir = next_dir;
        } else {
            break;
        }
    }
    (cur, dir)
}

enum Instruction {
    Left,
    Right,
    Move(usize),
}

fn do_path(map: &Map, instrs: &Vec<Instruction>, strat: StepStrategy, verbose: bool) {
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
                (coord, facing) = do_move(map, coord, facing, *amt, strat);
                if verbose {map.print(&coord)}
            }
        }
    }
    println!("Final row={}, column={}, facing={}", coord.1, coord.0, facing);
    println!("Password: {}", (coord.1 + 1) * 1000 + (coord.0 + 1) * 4 + facing as isize);
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

fn main() {
    let mut filename = "input.txt";
    let mut verbose = false;
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
        verbose = true;
    }
    let (map, instrs) = read_input(filename).unwrap();
    do_path(&map, &instrs, StepStrategy::Flat, false);
    do_path(&map, &instrs, StepStrategy::Cube, verbose);
}
