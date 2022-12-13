use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::error::Error;
use std::env;
use regex::{Regex, Match};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy)]
struct Item {
    id: u32,
    worry_level: usize,
}
#[derive(Clone, Copy)]
enum Operation {
    Plus(usize),
    Times(usize),
    Square,
}
#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: usize,
    inspect_count: usize,
    true_destination: usize,
    false_destination: usize,
}

impl Into<String> for &Item {
    fn into(self) -> String {
        format!("({}: {})", self.id, self.worry_level)
    }
}

fn single_num(s: &str, re: &Regex) -> MyResult<usize> {
    let mat: Match = re.find(s).ok_or::<Box<dyn Error>>(format!("Missing number in line \"{}\"", s).into())?;
    Ok(usize::from_str_radix(mat.as_str(), 10)?)
}

fn read_monkeys(filename: &str) -> MyResult<Vec<Monkey>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let mut id = 0;
    file.read_to_string(&mut contents)?;

    let mut monkeys: Vec<Monkey> = Vec::new();
    for block in contents.split("\n\n") {
        let lines: Vec<&str> = block.split("\n").collect();
        let num_re = Regex::new(r"\d+").unwrap();

        // Get items
        let mut items: Vec<Item> = Vec::new();
        for mat in num_re.find_iter(lines[1]) {
            let worry_level = usize::from_str_radix(mat.as_str(), 10)?;
            items.push(Item{id, worry_level});
            id += 1;
        }

        // Get operation and arg
        let operation = match num_re.find(lines[2]) {
            None => Operation::Square,
            Some(mat) => {
                let oparg = usize::from_str_radix(mat.as_str(), 10)?;
                if lines[2].contains('+') {
                    Operation::Plus(oparg)
                } else if lines[2].contains('*') {
                    Operation::Times(oparg)
                } else {
                    return Err("Missing operation".into());
                }
            }
        };

        let test = single_num(lines[3], &num_re)?;
        let true_destination = single_num(lines[4], &num_re)?;
        let false_destination = single_num(lines[5], &num_re)?;
        monkeys.push(Monkey{items, operation, test, true_destination, false_destination,
                            inspect_count: 0});
    }

    Ok(monkeys)
}

fn display_monkeys(monkeys: &Vec<Monkey>, round: u32) {
    println!("After round {}:", round);
    for (i, mon) in monkeys.iter().enumerate() {
        let items_str: Vec<String> = mon.items.iter().map(|v| v.into()).collect();
        println!("Monkey {}: {}", i, items_str.join(", "));
    }
}

fn display_monkey_counts(monkeys: &Vec<Monkey>) {
    for (i, mon) in monkeys.iter().enumerate() {
        println!("Monkey {}: inspected {} items", i, mon.inspect_count);
    }
    let mut vals: Vec<usize> = monkeys.iter().map(|m| m.inspect_count).collect();
    vals.sort();
    println!("Monkey business factor: {}", vals[vals.len() - 2] * vals[vals.len() - 1]);
}

fn do_monkey_round(monkeys: &mut Vec<Monkey>, round: u32, verbose: bool, worry_by_three: bool) {
    let mut modulo: usize = 1;
    for monkey in monkeys.iter() {
        modulo *= monkey.test;
    }
    for i in 0..monkeys.len() {
        let mut new_destinations: Vec<(Item, usize)> = Vec::new();
        for item in &monkeys[i].items {
            let mut newitem = *item;
            newitem.worry_level = match monkeys[i].operation {
                Operation::Plus(val) => newitem.worry_level + val as usize,
                Operation::Times(val) => newitem.worry_level * val as usize,
                Operation::Square => newitem.worry_level.pow(2),
            };
            if worry_by_three {
                newitem.worry_level = newitem.worry_level / 3;
            } else {
                newitem.worry_level %= modulo;
            }
            let idx = if newitem.worry_level % monkeys[i].test == 0 {
                monkeys[i].true_destination
            } else {
                monkeys[i].false_destination
            };
            new_destinations.push((newitem, idx));
        }
        monkeys[i].inspect_count += monkeys[i].items.len();
        monkeys[i].items.clear();
        for (item, newidx) in new_destinations {
            monkeys[newidx].items.push(item);
        }
    }
    if verbose {
        display_monkeys(monkeys, round);
    }
}

fn main() {
    let mut filename = "input.txt";
    let mut rounds = 20;
    let args: Vec<String> = env::args().collect();
    let mut verbose = false;
    if args.len() >= 2 {
        filename = &args[1];
        verbose = true;
    }
    if args.len() >= 3 {
        rounds = u32::from_str_radix(&args[2], 10).unwrap();
        verbose = true;
    }
    let mut monkeys = read_monkeys(filename).unwrap();
    let mut monkeys_backup = monkeys.clone();
    for round in 1..=rounds {
        do_monkey_round(&mut monkeys, round, verbose, true);
    }
    display_monkey_counts(&monkeys);
    println!("Now allowing worry to become ridiculous! 10000 iterations:");
    for round in 1..=10000 {
        do_monkey_round(&mut monkeys_backup, round, false, false);
    }
    display_monkey_counts(&monkeys_backup);
}
