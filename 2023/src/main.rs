use std::env;
use std::iter::Iterator;
use std::time::Instant;

use util::RunResult;

mod arr;
mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

type Runner = fn(&str) -> RunResult;
struct TestCase(Runner, Option<isize>, Option<isize>, Option<isize>, Option<isize>);

const DAYS: &[TestCase] = &[
    /* Part 1 uses two different examples, easiest to stick to example 1 */
    TestCase(day01::run, Some(142), Some(142), Some(54708), Some(54087)),
    TestCase(day02::run, Some(8), Some(2286), Some(1931), Some(83105)),
    TestCase(day03::run, Some(4361), Some(467835), Some(498559), Some(72246648)),
    TestCase(day04::run, Some(13), Some(30), Some(20407), Some(23806951)),
    TestCase(day05::run, Some(35), Some(46), Some(621354867), Some(15880236)),
    TestCase(day06::run, Some(288), Some(71503), Some(3316275), Some(27102791)),
];

fn run_one(case: &TestCase, fln: &str, expected: Option<(Option<isize>, Option<isize>)>) -> bool {
    let start = Instant::now();
    let res = case.0(fln);
    let elapsed = start.elapsed();
    match res {
        Err(e) => {
            println!("❌ - {}\n{}.{:03}s", e, elapsed.as_secs(), elapsed.subsec_millis());
            false
        }
        Ok((p1, p2)) => {
            if let Some((e1, e2)) = expected {
                if p1 == e1 && p2 == e2 {
                    println!("✅ in {}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
                    true
                } else {
                    println!("❌ - expected {:?}, {:?}, got {:?}, {:?}\n{}.{:03}s", e1, e2, p1, p2, elapsed.as_secs(), elapsed.subsec_millis());
                    false
                }
            } else {
                println!("✅ in {}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
                true
            }
        }
    }
}

fn do_run(dayno: u32, case: &TestCase, example: &str, fln: &str) -> usize {
    println!("==> Day {} <==", dayno);
    let mut fails = 0;
    println!(" -> Example:");
    if !run_one(case, example, Some((case.1, case.2))) { fails += 1 }
    println!(" -> Puzzle:");
    if !run_one(case, fln, Some((case.3, case.4))) { fails += 1 }
    println!();
    fails
}

fn default_input(dayno: u32) -> String {
    format!("src/day{:02}/input.txt", dayno)
}

fn example_input(dayno: u32) -> String {
    format!("src/day{:02}/example.txt", dayno)
}

fn run_all() -> usize {
    let mut fails = 0;
    let start = Instant::now();
    for (i, runner) in DAYS.iter().enumerate() {
        let i = i as u32 + 1;
        fails += do_run(i, &runner, &example_input(i), &default_input(i));
    }
    let elapsed = start.elapsed();
    println!("All tests completed.");
    if fails == 0 {
        println!("✅ in {}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
    } else {
        println!("❌ {} failed in {}.{:03}s", fails, elapsed.as_secs(), elapsed.subsec_millis());
    }
    fails
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "all" {
        run_all();
    } else {
        let day = u32::from_str_radix(&args[1], 10).unwrap();
        let case = &DAYS[day as usize - 1];

        if args.len() >= 3 {
            run_one(case, &args[2], None);
        } else {
            run_one(case, &default_input(day), Some((case.3, case.4)));
        }
    }
}
