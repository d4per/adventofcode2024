
use regex::Regex;

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));


    let re = Regex::new(r"\w\w\w(?<name>\w+)").unwrap();

    for line in lines {
        let Some(capture) = re.captures(line) else {
            println!("no match");
            return;
        };
        println!("capture: {:?}", capture.name("name"));
    }

}