use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;
use std::vec::Vec;

use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

struct Blueprint {
    ore_bot_cost_ore: u32,
    clay_bot_cost_ore: u32,
    obsidian_bot_cost_ore: u32,
    obsidian_bot_cost_clay: u32,
    geode_bot_cost_ore: u32,
    geode_bot_cost_obsidian: u32,
}
struct BlueprintCorollary {
    max_ore_bots: u32,
    max_clay_bots: u32,
    max_obsidian_bots: u32,
}

fn read_blueprints(filename: &str) -> MyResult<Vec<Blueprint>> {
    let reader = BufReader::new(File::open(filename)?);
    let re = Regex::new(r"\d+").unwrap();
    let mut res: Vec<_> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let vals: Vec<_> = re.find_iter(&line).map(|m| u32::from_str_radix(m.as_str(), 10).unwrap()).collect();
        assert_eq!(vals.len(), 7);
        res.push(Blueprint{
            ore_bot_cost_ore: vals[1],
            clay_bot_cost_ore: vals[2],
            obsidian_bot_cost_ore: vals[3],
            obsidian_bot_cost_clay: vals[4],
            geode_bot_cost_ore: vals[5],
            geode_bot_cost_obsidian: vals[6],
        })
    }
    Ok(res)
}

#[derive(Clone, Hash)]
struct State {
    minute: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_bots: u32,
    clay_bots: u32,
    obsidian_bots: u32,
    geode_bots: u32,
}

fn do_bot_production(state: &mut State) {
    state.ore += state.ore_bots;
    state.clay += state.clay_bots;
    state.obsidian += state.obsidian_bots;
    state.geode += state.geode_bots;
    state.minute -= 1;
}

fn maximize_geodes_rec(bp: &Blueprint, bpc: &BlueprintCorollary, state: &State) -> u32 {
    let mut best: Option<u32> = None;

    if state.ore_bots > 0 && state.ore_bots < bpc.max_ore_bots {
        let mut state = state.clone();
        while state.minute > 0 {
            if state.ore >= bp.ore_bot_cost_ore {
                do_bot_production(&mut state);
                state.ore_bots += 1;
                state.ore -= bp.ore_bot_cost_ore;
                let opt = maximize_geodes_rec(bp, bpc, &state);
                best = Some(best.map_or(opt, |b| b.max(opt)));
                break;
            } else {
                do_bot_production(&mut state);
            }
        }
    }
    if state.ore_bots > 0 && state.clay_bots < bpc.max_clay_bots {
        let mut state = state.clone();
        while state.minute > 0 {
            if state.ore >= bp.clay_bot_cost_ore {
                do_bot_production(&mut state);
                state.clay_bots += 1;
                state.ore -= bp.clay_bot_cost_ore;
                let opt = maximize_geodes_rec(bp, bpc, &state);
                best = Some(best.map_or(opt, |b| b.max(opt)));
                break;
            } else {
                do_bot_production(&mut state);
            }
        }
    }
    if state.ore_bots > 0 && state.clay_bots > 0 && state.obsidian_bots < bpc.max_obsidian_bots {
        let mut state = state.clone();
        while state.minute > 0 {
            if state.ore >= bp.obsidian_bot_cost_ore && state.clay >= bp.obsidian_bot_cost_clay {
                do_bot_production(&mut state);
                state.obsidian_bots += 1;
                state.ore -= bp.obsidian_bot_cost_ore;
                state.clay -= bp.obsidian_bot_cost_clay;
                let opt = maximize_geodes_rec(bp, bpc, &state);
                best = Some(best.map_or(opt, |b| b.max(opt)));
                break;
            } else {
                do_bot_production(&mut state);
            }
        }
    }
    if state.ore_bots > 0 && state.obsidian_bots > 0 {
        let mut state = state.clone();
        while state.minute > 0 {
            if state.ore >= bp.geode_bot_cost_ore && state.obsidian >= bp.geode_bot_cost_obsidian {
                do_bot_production(&mut state);
                state.geode_bots += 1;
                state.ore -= bp.geode_bot_cost_ore;
                state.obsidian -= bp.geode_bot_cost_obsidian;
                let opt = maximize_geodes_rec(bp, bpc, &state);
                best = Some(best.map_or(opt, |b| b.max(opt)));
                break;
            } else {
                do_bot_production(&mut state);
            }
        }
    }

    match best {
        Some(v) => v,
        None => {
            let mut state = state.clone();
            while state.minute > 0 {
                do_bot_production(&mut state);
            }
            state.geode
        }
    }
}

fn maximize_geodes(bp: &Blueprint, minutes: u32) -> u32 {
    let bpc = BlueprintCorollary{
        max_ore_bots: bp.clay_bot_cost_ore.max(bp.obsidian_bot_cost_ore.max(bp.geode_bot_cost_ore)),
        max_clay_bots: bp.obsidian_bot_cost_clay,
        max_obsidian_bots: bp.geode_bot_cost_obsidian,
    };
    maximize_geodes_rec(bp, &bpc, &State{
        minute: minutes,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,
    })
}

fn main() {
    let mut filename = "input.txt";
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
    }
    let bps = read_blueprints(filename).unwrap();
    let mut total_quality = 0;
    for (i, bp) in bps.iter().enumerate() {
        let i = i + 1;
        let geodes = maximize_geodes(bp, 24);
        let quality = i * geodes as usize;
        total_quality += quality;
        println!("[{}]: max geodes (24min): {} quality: {}", i, geodes, quality);
    }
    println!("Total quality: {}", total_quality);

    let mut product = 1;
    for i in 0..3 {
        let geodes = maximize_geodes(&bps[i], 32);
        println!("[{}]: max geodes (32min): {}", i + 1, geodes);
        product *= geodes;
    }
    println!("For the first 3 blueprints, product of the 32-minute geode quantities is: {}", product);
}
