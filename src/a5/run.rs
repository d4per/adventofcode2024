use std::cmp::Ordering;
use std::collections::HashSet;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));

    //find order rules
    let mut binding = lines.iter();
    let mut iter = binding.by_ref();
    let order_rules = iter.take_while(|line| (**line) != "").map(|line| {
        let mut split = line.split("|");
        let left = split.next().unwrap().parse::<i64>().unwrap();
        let right = split.next().unwrap().parse::<i64>().unwrap();
        (left, right)
    }).collect::<HashSet<(i64, i64)>>();

    println!("{:?}", order_rules);

    let number_lists = iter.map(|line| {
        let mut split = line.split(",");
        let numbers = split.map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        numbers
    }).collect::<Vec<Vec<i64>>>();

    println!("{:?}", number_lists);

    let mut unordered_list = Vec::<Vec<i64>>::new();
    
    let mut  num_sum = 0;
    for number_list in number_lists {
        let mut in_order = true;
        for two_numbers in number_list.iter().tuple_windows::<(&i64, &i64)>() {
            if !order_rules.contains(&(*two_numbers.0, *two_numbers.1)) {
                in_order = false;
                unordered_list.push(number_list.clone());
                break;
            }
        }
        if in_order {
            let middle_number = number_list.get(number_list.len()/2).unwrap();
            println!("{}", middle_number);
            num_sum += middle_number;
        }
    }
    
    
    println!("part 1 tot sum {}", num_sum);
    
    
    let mut num_sum2 = 0;
    for unordered_list in unordered_list {
        let sorted = unordered_list.iter().sorted_by(|a, b| {
            if order_rules.contains(&(**a, **b)) {
                Ordering::Less
            } else if order_rules.contains(&(**b, **a)){
                Ordering::Greater
            } else {
                println!("eq");
                Ordering::Equal
            }
        }).collect::<Vec<&i64>>();
        println!("sorted list {:?}", sorted);
        let middle_number = sorted.get(sorted.len()/2).unwrap();
        println!("{}", middle_number);
        num_sum2 += **middle_number;
        
    }
    println!("part 2 tot sum {}", num_sum2);
    
}