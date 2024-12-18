use std::collections::{HashMap, VecDeque};
use regex::Regex;


struct SMap {
    map: [char; 71*71],
    width: i64,
    height: i64,

}


impl SMap {
    fn set(&mut self, x: i64, y: i64, c: char) {
        self.map[(x+y*self.width) as usize] = c;
    }

    fn get(&self, x: i64, y:i64) -> char {
        self.map[(x+y*self.width) as usize]
    }

    fn find_best_path(&self, start: (i64, i64), end: (i64, i64)) -> u64 {
        let directions = [(-1i64,0i64), (1,0), (0,-1), (0,1)];
        let mut current_scores: HashMap<(i64,i64), u64> = HashMap::new();

        let mut queue: VecDeque<(i64,i64, u64)> = VecDeque::new();
        queue.push_back((start.0, start.1, 0));
        while(!queue.is_empty()) {
            let (x,y, score) = queue.pop_front().unwrap();
            if self.get(x,y) == '#' {
                continue;
            }

            let current_score = current_scores.get(&(x,y));
            if current_score.is_none() || *current_score.unwrap() > score {
                current_scores.insert((x,y), score);
                for (dx, dy) in directions {
                    let new_x = x + dx;
                    let new_y = y + dy;
                    let new_score = score +1;
                    if (new_x >= 0 && new_y >=0 && new_x < self.width && new_y < self.height) {
                        queue.push_back((new_x, new_y, new_score));
                    }
                }
            }
        }

        *current_scores.get(&end).unwrap()
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0.. self.width {
                print!("{}", self.map[(x+y*self.width) as usize]);
            }
            println!();
        }

    }
}

fn main() {
    let coords: Vec<(i64,i64)> = include_str!("input.txt")
        .split("\n")
        .map(|line| {
            let mut s = line.split(",");
            let x: i64 = s.next().unwrap().parse().unwrap();
            let y: i64 = s.next().unwrap().parse().unwrap();
            (x,y)
        }).collect();

    let mut map = SMap {
        map: ['.'; 71*71],
        width: 71,
        height: 71,
    };

    for i in 0..coords.len() {
        let limited_coords = &coords[0..i];

        for (x, y) in limited_coords.iter() {
            map.set(*x, *y, '#');
        }

        //map.print();

        let start = (0, 0);
        let end = (70, 70);
        let shortest_path = map.find_best_path(start, end);
        // println!("part 1: {shortest_path}");
        // //140
        // //244
        // //302
        println!("part 2: {i} {shortest_path}");

    }
}