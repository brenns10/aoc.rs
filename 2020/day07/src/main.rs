use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::HashMap;
use std::result::Result;

struct ColorAndCount {
    color: String,
    count: usize,
}

type Rules = HashMap<String, Vec<ColorAndCount>>;

fn read_rules(filename: &str) -> Result<Rules, String> {
    let f = File::open(filename).map_err(|e| e.to_string())?;
    let br = BufReader::new(f);
    let expr = Regex::new("^([a-z]+ [a-z]+) bags contain \
                          (no other bags|(\\d+ [a-z]+ [a-z]+ bags?, )*\\d+ [a-z]+ [a-z]+ bags?)\\.$")
            .unwrap();
    let inner = Regex::new("(\\d+) ([a-z]+ [a-z]+) bags?")
            .unwrap();
    let mut rules: Rules = HashMap::new();
    for line in br.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let c = expr.captures(&line).ok_or(format!("no match in line {}", line))?;
        let color = c.get(1).unwrap().as_str().to_string();
        let rules_str = c.get(2).unwrap().as_str();
        let mut vec: Vec<ColorAndCount> = Vec::new();
        for c in inner.captures_iter(rules_str) {
            let count: usize = c.get(1)
                                .unwrap()
                                .as_str()
                                .parse()
                                .map_err(|_| format!("bad int in rule"))?;
            let rule_color = c.get(2).unwrap().as_str().to_string();
            vec.push(ColorAndCount{color: rule_color, count: count});
        }
        rules.insert(color, vec);
    }
    Ok(rules)
}

enum VisitState {
    NotSeen,
    Visiting,
    Seen
}

fn topologic_sort_<'a>(color: &'a str, rules: &'a Rules, state: &mut HashMap<&'a str, VisitState>, output: &mut Vec<String>) -> Option<()> {
    let this_state = state.get(color).unwrap_or(&VisitState::NotSeen);
    match this_state {
        VisitState::Visiting => None,
        VisitState::Seen => Some(()),
        VisitState::NotSeen => {
            state.insert(color, VisitState::Visiting);
            for contained_rule in rules.get(color).unwrap() {
                topologic_sort_(&contained_rule.color, rules, state, output)?;
            }
            state.insert(color, VisitState::Seen);
            output.push(color.to_string());
            Some(())
        }
    }
}

fn topologic_sort(rules: &Rules) -> Option<Vec<String>> {
    let mut output: Vec<String> = Vec::new();
    let mut state: HashMap<&str, VisitState> = HashMap::new();

    for color in rules.keys() {
        topologic_sort_(color, rules, &mut state, &mut output)?;
    }

    Some(output)
}

fn count_reachable(rules: &Rules, sorted_colors: &Vec<String>, target: &str) -> usize {
    let mut count = 0;
    let mut is_reachable: HashMap<&str, bool> = HashMap::new();
    for color in sorted_colors {
        let mut this_is_reachable = color == target;
        for rule in rules.get(color).unwrap() {
            let color2 = rule.color.as_ref();
            this_is_reachable = this_is_reachable || *is_reachable.get(&color2).unwrap();
        }
        is_reachable.insert(&color, this_is_reachable);
        if this_is_reachable {
            println!("Could be in a {} bag", color);
            count += 1;
        }
    }
    count
}

fn count_bags_for(rules: &Rules, sorted_colors: &Vec<String>, target: &str) -> Option<usize> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for color in sorted_colors {
        let mut count: usize = 1;
        for rule in rules.get(color).unwrap() {
            let color2 = rule.color.as_ref();
            count += rule.count * counts.get(&color2).unwrap();
        }
        counts.insert(&color, count);
        if color == target {
            return Some(count);
        }
    }
    return None
}

fn main() {
    let rules = read_rules("input.txt").unwrap();
    let sorted_colors = topologic_sort(&rules).unwrap();
    let count = count_reachable(&rules, &sorted_colors, "shiny gold");
    println!("Shiny gold could be in {} bags (not including self)", count - 1);
    let bags_in_gold = count_bags_for(&rules, &sorted_colors, "shiny gold");
    println!("Shiny gold contains {} bags (not including self)", bags_in_gold.unwrap() - 1)
}
