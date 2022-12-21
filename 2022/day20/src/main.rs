use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn read_input(filename: &str) -> MyResult<Vec<(isize, usize)>> {
    let reader = BufReader::new(File::open(filename)?);
    Ok(reader.lines()
             .map(|s| isize::from_str_radix(&s.unwrap(), 10).unwrap())
             .enumerate()
             .map(|(k, v)| (v, k))
             .collect())
}

fn find_orig_index(file: &Vec<(isize, usize)>, index: usize) -> isize {
    for (i, (_, orig)) in file.iter().enumerate() {
        if *orig == index {
            return i as isize;
        }
    }
    panic!("Can't find original index {}", index);
}

fn print_arr(file: &Vec<(isize, usize)>){
    print!("  [");
    let mut first: bool = true;
    for elem in file.iter() {
        if first {
            print!("{}", elem.0);
            first = false;
        } else {
            print!(", {}", elem.0);
        }
    }
    print!("]\n")
}

fn do_mix(file: &mut Vec<(isize, usize)>, verbose: bool) {
    let len = file.len() as isize;
    for orig_idx in 0..file.len() {
        let cur_idx = find_orig_index(file, orig_idx);
        let mut shift = file[cur_idx as usize].0 % (len - 1);
        if shift == 0 { continue; } /* Simple really */
        if shift < 0 && shift.abs() > cur_idx {
            shift += len - 1;
        } else if shift > 0 && cur_idx + shift >= len {
            shift -= len - 1;
        }
        /* shift is now normalized so it will land within the index range, do
         * the shift */
        let new_idx = cur_idx + shift;
        let val = file[cur_idx as usize];
        file.remove(cur_idx as usize);
        file.insert(new_idx as usize, val);
        if verbose {
            println!("Value {} was originally at index {}, now at index {}.", val.0, orig_idx, cur_idx);
            println!("  Resolved the shift to: {}, new index is now: {}", shift, new_idx);
            print_arr(file);
        }
    }
}

fn find_coords(file: &Vec<(isize, usize)>) {
    for i in 0..file.len() {
        if file[i].0 == 0 {
            println!("1000th: {}, 2000th: {}, 3000th: {}", file[(i + 1000) % file.len()].0, file[(i + 2000) % file.len()].0, file[(i + 3000) % file.len()].0);
            let res = file[(i + 1000) % file.len()].0 + file[(i + 2000) % file.len()].0 + file[(i + 3000) % file.len()].0;
            println!("Sum: {}", res);
            return;
        }
    }
    println!("Error, couldn't find 0")
}

fn decrypt(file: &mut Vec<(isize, usize)>, key: isize, rounds: usize) {
    for val in file.iter_mut() {
        val.0 = val.0 * key;
    }
    for _ in 0..rounds {
        do_mix(file, false);
    }
}

fn main() {
    let mut filename = "input.txt";
    let mut verbose = false;
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        filename = &args[1];
        verbose = true;
    }
    let mut file = read_input(filename).unwrap();
    let mut part2 = file.clone();
    if verbose {
        println!("Original arrangement:");
        print_arr(&file);
    }
    do_mix(&mut file, verbose);
    find_coords(&file);

    println!("Doing part 2 decryption!");
    decrypt(&mut part2, 811589153, 10);
    find_coords(&part2);
}
