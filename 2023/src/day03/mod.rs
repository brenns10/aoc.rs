use std::collections::{HashMap, HashSet};

use crate::arr::{Arr2D, Point};
use crate::util::return_part1and2;
use crate::util::RunResult;

pub fn run(fln: &str) -> RunResult {
    let arr = Arr2D::read_file(fln)?;
    let mut part_number_sum = 0;
    let mut gear_to_numbers: HashMap<Point, Vec<u32>> = HashMap::new();
    let mut gears: HashSet<Point> = HashSet::new();
    for row in 0..arr.rows {
        let mut number: u32 = 0;
        let mut is_part = false;
        for col in 0..arr.cols {
            if let Some(val) = arr.get_at(row, col).to_digit(10) {
                number = number * 10 + val;
                for point in arr.adjacent_to(row, col) {
                    let cell = arr.get(point);
                    if !cell.is_digit(10) && cell != '.' {
                        is_part = true;
                    }
                    if cell == '*' {
                        gears.insert(point);
                    }
                }
            } else if number > 0 {
                if is_part {
                    part_number_sum += number;
                    for gear in gears.iter() {
                        if let Some(vec) = gear_to_numbers.get_mut(gear) {
                            vec.push(number);
                        } else {
                            gear_to_numbers.insert(*gear, vec![number]);
                        }
                    }
                }
                number = 0;
                is_part = false;
                gears.clear();
            }
        }
        if number > 0 && is_part {
            part_number_sum += number;
            for gear in gears.iter() {
                if let Some(vec) = gear_to_numbers.get_mut(gear) {
                    vec.push(number);
                } else {
                    gear_to_numbers.insert(*gear, vec![number]);
                }
            }
        }
        gears.clear();
    }
    println!("Part 1: {}", part_number_sum);

    let mut sum_ratios = 0;
    for (_, numbers) in gear_to_numbers.iter() {
        if numbers.len() == 2 {
            sum_ratios += numbers[0] * numbers[1];
        }
    }
    println!("Part 2: {}", sum_ratios);

    return_part1and2(part_number_sum as isize, sum_ratios as isize)
}
