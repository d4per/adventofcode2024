
use regex::Regex;

fn part1() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));

    print!("{:?}",lines);

    let width = lines[0].len();
    let height = lines.len();

    let mut sum_words = 0;
    for y in 0..height as isize {
        for x in 0..width as isize {
            for dy in  -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    sum_words += if find_word("XMAS", &lines, x,y, dx, dy) { 1 } else { 0 };
                }
            }

        }
    }
    println!("sum {}", sum_words);

}

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));
    
    let width = lines[0].len();
    let height = lines.len();
    
    let mut sum_words = 0;
    for y in 0..height as isize {
        for x in 0..width as isize {

            let w1a = find_word("MAS", &lines, x - 1, y -1, 1, 1);
            let w1b = find_word("SAM", &lines, x - 1, y -1, 1, 1);
            let w2a = find_word("MAS", &lines, x +1, y-1, -1, 1);
            let w2b = find_word("SAM", &lines, x +1, y-1, -1, 1);
            
            if (w1a || w1b) && (w2a || w2b) {
                sum_words += 1;
            }
        } 
    }
    println!("sum {}", sum_words);
    //4148
    //3810 too high
    //1905
    //2070
    //2046  <--- 
    
}

fn find_word(word: &str, lines: &Vec<&str>, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let width = lines[0].len();
    let height = lines.len();

    let mut xp = x;
    let mut yp = y;
    for i in 0..word.len() {
        if xp <0 || yp < 0 || xp >= width as isize || yp >= height as isize {
            return false;
        }

        let c = word.chars().nth(i).unwrap();
        let cc = lines[yp as usize].chars().nth(xp as usize).unwrap();
        if c != cc {
            return false;
        }
        xp += dx;
        yp += dy;
    }
    true
}