use std::env;
use std::iter::Iterator;
use std::time::Instant;

mod arr;

mod day01;
mod day02;
mod day03;

const DAYS: &[fn(&str) -> ()] = &[
    day01::run,
    day02::run,
    day03::run,
];

fn do_run(dayno: u32, runner_fn: &fn(&str) -> (), fln: &str) {
    println!("==> Day {} <==", dayno);
    let start = Instant::now();
    runner_fn(fln);
    let elapsed = start.elapsed();
    println!("✅ in {}.{:03}s\n", elapsed.as_secs(), elapsed.subsec_millis());
}

fn default_input(dayno: u32) -> String {
    format!("src/day{:02}/input.txt", dayno)
}

fn run_all() {
    for (i, runner) in DAYS.iter().enumerate() {
        let i = i as u32 + 1;
        do_run(i, &runner, &default_input(i));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "all" {
        run_all();
    } else {
        let day = u32::from_str_radix(&args[1], 10).unwrap();

        if args.len() >= 3 {
            do_run(day, &DAYS[day as usize - 1], &args[2]);
        } else {
            do_run(day, &DAYS[day as usize - 1], &default_input(day))
        }
    }
}
