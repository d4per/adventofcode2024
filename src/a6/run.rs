use std::collections::HashSet;
use regex::Regex;

fn part1() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));


    let mut guard_x = -1 as i64;
    let mut guard_y = -1 as i64;
    let mut current_direction = "^";
    //find guard position
    for (y, line) in lines.iter().enumerate() {
        let index = line.find(current_direction);
        match index {
            None => {}
            Some(x) => {
                guard_x = x as i64;
                guard_y = y as i64;
                break;
            }
        }
    }

    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    let mut guard_positions: HashSet<(i64,i64)> = HashSet::new();
    //move guard
    loop {
        guard_positions.insert((guard_x, guard_y));
        let mut new_x = 0;
        let mut new_y = 0;
        let mut new_direction = "-";
        match current_direction {
            "^" => {
                new_x = guard_x;
                new_y = guard_y - 1;
                new_direction = ">";
            },
            "v" => {
                new_x = guard_x;
                new_y = guard_y + 1;
                new_direction = "<";
            },
            "<" => {
                new_x = guard_x - 1;
                new_y = guard_y;
                new_direction = "^";
            },
            ">" => {
                new_x = guard_x + 1;
                new_y = guard_y;
                new_direction = "v";
            },
            _ => { panic!("unhandled case")}
        }

        if new_x<0 || new_x>=width || new_y<0 || new_y>=height {
            //done
            break;
        }
        let char_at_new_pos = lines[new_y as usize].chars().nth(new_x as usize).unwrap().to_string();
        if char_at_new_pos.eq("#") {
            current_direction = new_direction;
        } else {
            guard_x = new_x;
            guard_y = new_y;
        }
    }



    println!("part 1: {}", guard_positions.len());


}

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));


    let mut guard_x = -1 as i64;
    let mut guard_y = -1 as i64;
    let mut current_direction = "^";
    //find guard position
    for (y, line) in lines.iter().enumerate() {
        let index = line.find(current_direction);
        match index {
            None => {}
            Some(x) => {
                guard_x = x as i64;
                guard_y = y as i64;
                break;
            }
        }
    }

    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    
    
    let mut stuck_count = 0;
    for y in 0..height {
        for x in 0..width {
            let extra_obstecle = (x,y);
            let (squares, stuck) = simulate_guard_movements(&mut lines, guard_x, guard_y, &mut current_direction, extra_obstecle);
            if stuck {
                stuck_count += 1;
            }
        }
    }

    println!("part 2: {}", stuck_count);
}

fn simulate_guard_movements(lines: &Vec<&str>, start_guard_x: i64, start_guard_y: i64, start_direction: &str, extra_obstecle: (i64, i64)) -> (usize, bool){
    let mut guard_x = start_guard_x;
    let mut guard_y = start_guard_y;
    let mut current_direction = start_direction;
    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    let mut guard_positions: HashSet<(i64, i64)> = HashSet::new();
    let mut guard_positions_and_dir: HashSet<(i64, i64, &str)> = HashSet::new();
    let mut stuck_in_loop = false;
    //move guard
    loop {
        if guard_positions_and_dir.contains(&(guard_x, guard_y, current_direction)) {
            stuck_in_loop = true;
            break;
        }

        guard_positions.insert((guard_x, guard_y));
        guard_positions_and_dir.insert((guard_x, guard_y, current_direction));
        let mut new_x = 0;
        let mut new_y = 0;
        let mut new_direction = "-";
        match current_direction {
            "^" => {
                new_x = guard_x;
                new_y = guard_y - 1;
                new_direction = ">";
            },
            "v" => {
                new_x = guard_x;
                new_y = guard_y + 1;
                new_direction = "<";
            },
            "<" => {
                new_x = guard_x - 1;
                new_y = guard_y;
                new_direction = "^";
            },
            ">" => {
                new_x = guard_x + 1;
                new_y = guard_y;
                new_direction = "v";
            },
            _ => { panic!("unhandled case") }
        }

        if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
            //done
            break;
        }
        let char_at_new_pos = lines[new_y as usize].chars().nth(new_x as usize).unwrap().to_string();
        if char_at_new_pos.eq("#") || (new_x, new_y) == extra_obstecle {
            current_direction = new_direction;
        } else {
            guard_x = new_x;
            guard_y = new_y;
        }
    }

    (guard_positions.len(), stuck_in_loop)

}