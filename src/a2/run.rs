fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));


    println!("lines: {:?}", lines);

}