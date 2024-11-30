fn main() {
    let list = include_str!("input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    println!("list {:?}",list);

}