use std::collections::{HashSet, VecDeque};
use regex::Regex;

fn main() {
    let mut lines = Vec::<&str>::new();

    let map: Vec<Vec<char>> = include_str!("input.txt")
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    let mut start_positions: Vec<(i64, i64)> = Vec::new();

    for y in 0..map.iter().len() {
        for x in 0..map[0].len() {
            if map[y][x] == '0' {
                start_positions.push((x as i64, y as i64));
            }
        }
    }

    let mut sum_score = 0;
    for start in start_positions {
        sum_score += find_paths(&map, &start, 0);
    }
    println!("sum_score {}", sum_score);

}

fn find_paths(map: &Vec<Vec<char>>, start: &(i64, i64), current_elevation: usize) -> usize {
    let directions = [(-1i64,0i64), (1,0), (0, -1), (0, 1)];
    let mut queue: VecDeque<((i64, i64), usize)> = VecDeque::new();
    queue.push_back((*start, current_elevation));
    let map_width = map[0].len() as i64;
    let map_height = map.len() as i64;
    let mut found_paths = 0usize;
    let mut already_found  : HashSet<(i64, i64)> = HashSet::new();

    while !queue.is_empty() {
        let ((x, y), elevation) = queue.pop_back().unwrap();
        for d in directions {
            let xx = x + d.0;
            let yy = y + d.1;
            if xx < 0 || yy < 0 || xx >= map_width || yy >= map_height {
                continue;
            }
            let e_square: usize = (map[yy as usize][xx as usize]).to_string().parse().unwrap();
            if e_square == elevation + 1 {
                if e_square == 9 && !already_found.contains(&(xx,yy)) {
                    found_paths += 1;
                    //already_found.insert((xx,yy));
                } else {
                    queue.push_back(((xx,yy), e_square))
                }
            }
        }


    }


    found_paths
}