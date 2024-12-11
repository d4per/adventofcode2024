use std::cell::RefCell;
use std::collections::HashMap;
use regex::Regex;

fn part1() {

    let input = "41078 18 7 0 4785508 535256 8154 447";
    //let input = "125 17";  //example
    let mut numbers: Vec<u128> = input.split(" ").map(|s| s.parse().unwrap()).collect();


    for i in 0..25 {

        //println!("{:?}", &numbers);
        let mut new_numbers: Vec<u128> = Vec::new();

        for number in &numbers {
            let mut n_list = transform_one_number(*number);
            new_numbers.append(&mut n_list);

        }
    }

    let length: usize = numbers.len();

    println!("sum: {}", length)
    //1521121284083

}

fn main() {
    let input = "41078 18 7 0 4785508 535256 8154 447";
    //let input = "125 17";  //example
    let mut numbers: Vec<u128> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    let mut cache = Cache::new();
    let mut length = 0;
    for i in 0..75+1 {
        length = find_recursive(&mut cache, &numbers, i);
        println!("l {i} {}", length);
    }


    println!("sum: {}", length)

}

struct Cache {
    result_map: RefCell<HashMap<(u128, usize), usize>>
}



impl Cache {

    fn new() -> Cache {
        Cache {
            result_map: RefCell::new(HashMap::new())
        }
    }

    pub(crate) fn put(&self, number: u128, dept: usize, len: usize) {
        //println!();
        //println!("put {} {} {}", number, dept, len);
        self.result_map.borrow_mut().insert((number, dept), len);
    }

    fn get(&self, number: u128, depth: usize) -> Option<usize> {
        let map = self.result_map.borrow();
        println!("size: {}", map.len());
        let ans = map.get(&(number, depth));

        match ans {
            None => {None}
            Some(v) => { Some(v.clone())}
        }
    }
}



fn find_recursive(cache: &Cache, parent: &Vec<u128>, depth: usize) -> usize {
    if depth == 0 {
        return parent.len();
    }
    let mut sum_len = 0;
    for n in parent {
        let t_one = cache.get(*n, depth);
        let tr = match t_one {
            None => {
                print!("n");
                let len = find_recursive(cache, &transform_one_number(*n), depth - 1);
                cache.put(*n, depth, len);
                len
            }
            Some(tt) => {
                print!("f");
                tt
            }
        };


        sum_len += tr;
    }
    sum_len

}

fn transform_one_number(number: u128) -> Vec<u128> {
    //print!("{} ", number);
    let mut new_numbers = match number {
        0 => Vec::from([1]),
        n if (format!("{}", n).len() & 1) == 0 => {
            let s = format!("{}", n);
            let l = s.len();
            let s1 = &s[..(l / 2)];
            let s2 = &s[(l / 2)..l];
            Vec::from([s1.parse().unwrap(), s2.parse().unwrap()])
        },
        n => Vec::from([n * 2024]),
    };
    new_numbers
}

fn transform(numbers: &Vec<u128>) -> Vec<u128> {
    let mut result: Vec<u128> = Vec::new();

    for number in numbers {
        let mut new_numbers = match number {
            0 => Vec::from([1]),
            n if (format!("{}", n).len() & 1) == 0 => {
                let s = format!("{}", n);
                let l = s.len();
                let s1 = &s[..(l / 2)];
                let s2 = &s[(l / 2)..l];
                Vec::from([s1.parse().unwrap(), s2.parse().unwrap()])
            },
            n => Vec::from([n * 2024]),
        };

        result.append(&mut new_numbers);
    }


    result

}