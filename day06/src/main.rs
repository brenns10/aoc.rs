use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

struct Group {
    group_size: usize,
    any_letters: usize,
    all_letters: usize,
    letters: [usize; 26],
}

impl Group {
    fn new() -> Group {
        Group{
            group_size: 0,
            any_letters: 0,
            all_letters: 0,
            letters: [0; 26],
        }
    }

    fn set_all_letters(&mut self) {
        self.all_letters = 0;
        for cnt in self.letters.iter() {
            if *cnt == self.group_size {
                self.all_letters += 1;
            }
        }
    }

    fn update(&mut self, c: char) -> Result<(), String> {
        let idx: i32 = (c as i32) - ('a' as i32);
        if idx < 0 || idx >= 26 {
            return Err(format!("invalid character '{}'", c));
        }
        if self.letters[idx as usize] == 0 {
            self.any_letters += 1;
        }
        self.letters[idx as usize] += 1;
        Ok(())
    }
}

fn load_groups(filename: &str) -> Result<Vec<Group>, String> {
    let f = File::open(filename).map_err(|e| e.to_string())?;
    let br = BufReader::new(f);
    let mut groups: Vec<Group> = Vec::new();
    let mut thisgrp = Group::new();
    for line in br.lines() {
        let line = line.map_err(|e| e.to_string())?;
        for c in line.chars() {
            thisgrp.update(c)?;
        }
        if line.len() == 0 {
            thisgrp.set_all_letters();
            groups.push(thisgrp);
            thisgrp = Group::new();
        } else {
            thisgrp.group_size += 1;
        }
    }
    thisgrp.set_all_letters();
    groups.push(thisgrp);
    Ok(groups)
}

fn main() {
    println!("Hello, world!");
    let grps = load_groups("input.txt").unwrap();
    let mut any = 0;
    let mut all = 0;
    for grp in grps.iter() {
        any += grp.any_letters;
        all += grp.all_letters;
    }
    println!("Count where any in group said yes: {}", any);
    println!("Count where all in group said yes: {}", all);
}
