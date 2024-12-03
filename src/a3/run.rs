
use regex::Regex;

fn part1() {
    let mut lines = Vec::<&str>::new();

    let input = include_str!("input.txt");


    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    
    let mut sum = 0;
    for a in re.captures_iter(input) {
        println!("{:?}", a);
        let m1 = a.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let m2 = a.get(2).unwrap().as_str().parse::<u64>().unwrap();
        println!("{} {} ", m1, m2);
        sum += m1 * m2;
    }
    println!("sum: {}", sum);

}

fn main() {
    let mut lines = Vec::<&str>::new();

    let input = include_str!("input.txt");


    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();


    let mut sum = 0;
    let mut in_do = true;
    for a in re.captures_iter(input) {
        println!("{:?}", a);
        let expr = a.get(0).unwrap().as_str();
        
        match (in_do, expr) {
            (_, e) if e.starts_with("do(") => {
                in_do = true;
            },
            (_, e) if e.starts_with("don't(") => {
                in_do = false;
            },
            (true, e) if e.starts_with("mul") => {
                let m1 = a.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let m2 = a.get(2).unwrap().as_str().parse::<u64>().unwrap();
                println!("{} {} ", m1, m2);
                sum += m1 * m2;
            },
            (false, e) if e.starts_with("mul") => {
                //do nothing
            },
            _ => panic!("unhandled case")
        };
        

    }
    println!("sum: {}", sum);

}