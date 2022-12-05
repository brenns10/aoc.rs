use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::result::Result;
use std::vec::Vec;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn get_elf_calories() -> MyResult<Vec<u32>> {
    let mut input = File::open("input.txt")?;
    let mut input_string = String::new();
    let mut current_elf_cals: u32 = 0;
    let mut elf_cal_list: Vec<u32> = Vec::new();
    input.read_to_string(&mut input_string)?;

    for line in input_string.split("\n") {
        match line {
            "" => {
                elf_cal_list.push(current_elf_cals);
                current_elf_cals = 0;
            }
            &_ => {
                current_elf_cals += u32::from_str_radix(line, 10)?;
            }
        }
    }
    if current_elf_cals > 0 {
        elf_cal_list.push(current_elf_cals);
    }
    Ok(elf_cal_list)
}

fn main() {
    let mut cal_list = get_elf_calories().unwrap();
    cal_list.sort_by(|a, b| b.cmp(a));

    println!("Elf with maximum calories has: {}", cal_list[0]);
    println!("Top 3 elves have: {}", cal_list[0] + cal_list[1] + cal_list[2])
}
