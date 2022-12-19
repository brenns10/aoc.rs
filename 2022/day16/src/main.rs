use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::result::Result;
use std::vec::Vec;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
    shortest_paths: HashMap<String, u32>,
}

fn read_valves(filename: &str) -> MyResult<HashMap<String, Valve>> {
    let reader = BufReader::new(File::open(filename)?);
    let mut res: HashMap<String, Valve> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let name = String::from(tokens[1]);
        let rate_str = &tokens[4];
        let rate_str = &rate_str[5..rate_str.len() - 1];
        let rate = u32::from_str_radix(rate_str, 10)?;
        let mut tunnels: Vec<String> = Vec::new();
        for tunnel in &tokens[9..] {
            if tunnel.len() == 3 {
                tunnels.push(String::from(&tunnel[..tunnel.len() - 1]));
            } else {
                tunnels.push(String::from(*tunnel));
            }
        }
        res.insert(name.clone(), Valve{name, rate, tunnels, shortest_paths: HashMap::new()});
    }
    Ok(res)
}

fn do_shortest_path(start: &str, graph: &mut HashMap<String, Valve>) {
    let mut path_lengths: HashMap<String, u32> = HashMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    path_lengths.insert(String::from(start), 0);
    queue.push_front(String::from(start));

    while !queue.is_empty() {
        let name = queue.pop_back().unwrap();
        let valve = graph.get(&name).unwrap();
        let cur_path = *path_lengths.get(&name).unwrap();
        let tunnels = valve.tunnels.clone();
        for tunnel in tunnels.iter() {
            if path_lengths.contains_key(tunnel) {
                continue;
            }
            if let Some(_) = graph.get(tunnel) {
                queue.push_front(tunnel.clone());
                path_lengths.insert(tunnel.clone(), cur_path + 1);
            } else {
                panic!("No next line for {}", tunnel);
            }
        }
    }
    let valve = graph.get_mut(start).unwrap();
    valve.shortest_paths = path_lengths;
}

fn do_all_shortest_path(graph: &mut HashMap<String, Valve>) {
    let names: Vec<String> = graph.keys().map(|v| String::from(v)).collect();
    for name in names {
        do_shortest_path(&name, graph);
    }
}

fn all_choices(start: &str, minutes_remain: u32, graph: &HashMap<String, Valve>, closed: &HashSet<String>) -> Vec<(String, u32, u32)> {
    let mut vec: Vec<(String, u32, u32)> = Vec::new();
    let start_valve = graph.get(start).unwrap();

    for tunnel in graph.values() {
        let path_len = *start_valve.shortest_paths.get(&tunnel.name).unwrap();
        if path_len + 1 >= minutes_remain {
            continue
        }
        if closed.contains(&tunnel.name) {
            continue
        }
        if tunnel.rate == 0 {
            continue
        }
        let score = (minutes_remain - path_len - 1) * tunnel.rate;
        vec.push((tunnel.name.clone(), score, path_len));
    }
    vec
}

fn closed_set_string(closed: &HashSet<String>) -> String {
    let mut v: Vec<&str> = closed.iter().map(|v| v.as_ref()).collect();
    v.sort();
    v.join("")
}

fn best_choice_rec(start: &str, minutes_remain: u32, graph: &HashMap<String, Valve>, closed: &HashSet<String>,
                   memoize: &mut HashMap<(String, String, u32), (u32, Vec<String>)>, depth: u32) -> (u32, Vec<String>) {
    let mut best: Option<(u32, Vec<String>)> = None;
    let key = (closed_set_string(closed), String::from(start), minutes_remain);
    if let Some(val) = memoize.get(&key) {
        return val.clone();
    }
    //let pfx = " ".repeat(depth as usize);
    //println!("{}Finding best choice: {} minutes at {}", pfx, minutes_remain, start);
    for (next, max_score, minutes) in all_choices(start, minutes_remain, graph, closed) {
        //println!("{}Option: {}, {} minutes away", pfx, next, minutes);
        let next_rem = minutes_remain - 1 - minutes;
        let mut closed_next = closed.clone();
        closed_next.insert(next.clone());
        let ( mut score, mut moves) = best_choice_rec(&next, next_rem, graph, &closed_next, memoize, depth + 1);
        score += max_score;
        moves.insert(0, next.clone());
        best = match best {
            None => Some((score, moves)),
            Some((other_score, other_moves)) => {
                if score > other_score {
                    Some((score, moves))
                } else {
                    Some((other_score, other_moves))
                }
            }
        }
    }
    let best = best.unwrap_or((0, Vec::new()));
    memoize.insert(key, best.clone());
    //println!("{}Best choice was: {:?}", pfx, best.1);
    best
}

fn best_choice(graph: &HashMap<String, Valve>) -> (u32, Vec<String>) {
    let closed: HashSet<String> = HashSet::new();
    let mut memoize: HashMap<(String, String, u32), (u32, Vec<String>)> = HashMap::new();
    best_choice_rec("AA", 30, graph, &closed, &mut memoize, 0)
}

fn main() {
    let mut filename: &str = "input.txt";
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let mut valves = read_valves(filename).unwrap();
    do_all_shortest_path(&mut valves);
    let (score, sequence) = best_choice(&valves);
    println!("Max score: {}", score);
    println!("Sequence: {:?}", sequence);
}
