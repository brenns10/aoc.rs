use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::result::Result;

struct Group {
    group_size: usize,
    distinct_letters: usize,
    letters: [bool; 26],
}

fn update_group(group: &mut Group, c: char) -> Result<(), String> {
    let idx: i32 = (c as i32) - ('a' as i32);
    if idx < 0 || idx >= 26 {
        return Err(format!("invalid character '{}'", c));
    }
    if !group.letters[idx as usize] {
        group.distinct_letters += 1;
    }
    group.letters[idx as usize] = true;
    Ok(())
}

fn load_groups(filename: &str) -> Result<Vec<Group>, String> {
    let f = File::open(filename).map_err(|e| e.to_string())?;
    let br = BufReader::new(f);
    let mut groups: Vec<Group> = Vec::new();
    let mut thisgrp = Group{group_size: 0, distinct_letters: 0, letters: [false; 26]};
    for line in br.lines() {
        let line = line.map_err(|e| e.to_string())?;
        for c in line.chars() {
            update_group(&mut thisgrp, c)?;
        }
        if line.len() == 0 {
            groups.push(thisgrp);
            thisgrp = Group{group_size: 0, distinct_letters: 0, letters: [false; 26]};
        } else {
            thisgrp.group_size += 1;
        }
    }
    groups.push(thisgrp);
    Ok(groups)
}

fn main() {
    println!("Hello, world!");
    let grps = load_groups("input.txt").unwrap();
    let mut distinct = 0;
    for grp in grps.iter() {
        distinct += grp.distinct_letters;
    }
    println!("Total distinct yes of all groups: {}", distinct);
}
