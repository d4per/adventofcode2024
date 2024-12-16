use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use regex::Regex;


struct SMap {
    map: Vec<char>,
    width: i64,
    height: i64,
}

impl SMap {
    fn get(&self, x: i64, y: i64) -> char {
        self.map[(x + y * self.width) as usize]
    }
}

fn main() {

    let lines: Vec<&str> = include_str!("input.txt")
        .split("\n")
        .collect();
    let width = lines[0].len();
    let height = lines.len();


    let map: Vec<char> = lines
        .into_iter().map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let start_index = map.clone().into_iter().enumerate().find(|(i, c)| *c == 'S').unwrap();
    let end_index = map.clone().into_iter().enumerate().find(|(i, c)| *c == 'E').unwrap();
    let start_pos = ((start_index.0  % width) as i64, (start_index.0 / height) as i64);
    let end_pos = ((end_index.0  % width) as i64, (end_index.0 / height) as i64);
    let smap = SMap {
        map, width: width as i64, height: height as i64
    };


    println!("{:?}", start_pos);

    let start_direction = '>';

    find_best_path(&smap, start_pos, start_direction, end_pos);

}

#[derive(Debug)]
struct Sqare {
    best_score: Cell<usize>,
}


impl Sqare {
    fn new() -> Sqare {
        Sqare {
            best_score: Cell::new(usize::MAX)
        }
    }

    fn set_best_score(&self, score: usize) {
        self.best_score.set(score);
    }

    fn get_best_score(&self) -> usize {
        self.best_score.get()
    }
}

fn find_best_path(map: &SMap, start: (i64, i64), start_dir: char, end: (i64, i64)) {
    let mut squares : HashMap<(i64, i64, char), Sqare> = HashMap::new();
    let mut current_pos = start;
    let mut current_dir = start_dir;
    let mut queue: VecDeque<(i64,i64, char, usize)> = VecDeque::new();
    queue.push_back((current_pos.0, current_pos.1, current_dir, 0));

    while !queue.is_empty() {
        let (x,y,c, score) = queue.pop_front().unwrap();
        let map_square = map.get(x, y);
        if map_square == '#' {
            continue;
        }

        //insert if empty
        match squares.get(&(x,y,c)) {
            None => {
                let square = Sqare::new();
                squares.insert((x,y,c), square);

            }
            Some(_) => {
            }
        }
        let s = squares.get(&(x,y,c)).unwrap();

        if s.get_best_score() > score {
            s.set_best_score(score);

            match c {
                '<' => {
                    queue.push_back((x-1, y, c, score + 1));
                    queue.push_back((x, y, '^', score + 1000));
                    queue.push_back((x, y, 'v', score + 1000));
                },
                '^' => {
                    queue.push_back((x, y-1, c, score + 1));
                    queue.push_back((x, y, '<', score + 1000));
                    queue.push_back((x, y, '>', score + 1000));
                },
                '>' => {
                    queue.push_back((x+1, y, c, score + 1));
                    queue.push_back((x, y, '^', score + 1000));
                    queue.push_back((x, y, 'v', score + 1000));
                },
                'v' => {
                    queue.push_back((x, y+1, c, score + 1));
                    queue.push_back((x, y, '<', score + 1000));
                    queue.push_back((x, y, '>', score + 1000));
                },
                _ => {
                    panic!()
                }
            }

        }
    }
    let best_end_square = squares.iter().find(|(((x,y,c),s))| *x == end.0 && *y == end.1)
        .map(|(_, s)| s)
        .into_iter().min_by_key(|s| s.get_best_score());

    println!("part 1: {:?}", best_end_square);
    //524 not right
    //144580
    //145580
    //143580

}