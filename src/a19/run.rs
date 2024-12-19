use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));

    let towel_types: Vec<String> = lines[0].split(",")
        .map(|t| t.trim().to_string())
        .sorted_by_key(|t| (t.len() as i64))
        .collect();

    let combined_towels = &lines[2..lines.len()];

    println!("{:?} {:?}", &towel_types.len(), &combined_towels.len());



    // let count = combined_towels.iter()
    //     .filter(|combined_towel| is_possible_pattern(combined_towel, &towel_types))
    //     .count();

    //panic!();

    let mut count = 0;
    for combined_towel in combined_towels {
        println!("todo {combined_towel}");
        let cache = Cache {
            impossible_patterns: RefCell::new(HashSet::new())
        };
        if is_possible_pattern(combined_towel, &towel_types,0, &cache) {
            println!("possible {combined_towel}");
            count += 1;
        } else {
            println!("not possible {combined_towel}");
        }
    }

    println!("part 1: {count}");
    //400
}

struct Cache {
    impossible_patterns: RefCell<HashSet<String>>,
}

static mut a: usize = 0;

fn is_possible_pattern(pattern: &str, towels: &Vec<String>, level: usize, cache: &Cache) -> bool {

    unsafe {
        a += 1;
        if a % 1000000 == 0 {
            println!("{a}");
        }
    }


    if cache.impossible_patterns.borrow().contains(pattern) {
        print!("x");
        return false;
    }

    let mut is_possible = false;
    for t in towels {
        if pattern.starts_with(t) {
            let sub_pattern = &pattern[t.len().. pattern.len()];
            if level <= 4 {
                println!("level {level} {t}");
            }
            if sub_pattern.is_empty() || is_possible_pattern(sub_pattern, towels, level + 1, cache) {
                is_possible = true;
                break;
            }
        }
    }
    if !is_possible {
        cache.impossible_patterns.borrow_mut().insert(pattern.to_string());
    }
    is_possible
}
