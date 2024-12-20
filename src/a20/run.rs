use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

struct SMap {
    map: Vec<char>,
    width: i64,
    height: i64,
    cache: RefCell<HashMap<(i64, i64, usize), usize>>,
}

impl SMap {
    fn get(&self, x: i64, y: i64) -> char {
        self.map[(x + y * self.width) as usize]
    }

    fn find_paths(&mut self, start: (i64, i64), end: (i64, i64), num_cheats: usize, max_depth: usize) -> Vec<(usize)> {

        let directions = [(-1i64,0i64), (1,0), (0,-1), (0,1)];
        let mut queue: VecDeque<Runner> = VecDeque::new();
        let first_runner = Runner::new(start.0, start.1, num_cheats);
        queue.push_back(first_runner);
        let mut done_runners: Vec<(usize)> = Vec::new();
        while !queue.is_empty() {

            let runner = queue.pop_front().unwrap();
            if queue.len() % 10000 == 0 {
                println!("queue size {} {}", queue.len(), runner.already_visited.len());
            }

            if runner.already_visited.len() >= max_depth {
                continue;
            }

            if runner.current_x <0 || runner.current_y <0 || runner.current_x>= self.width || runner.current_y>=self.height {
                continue;
            }

            if runner.has_already_visited_current() {
                continue;
            }

            if self.cache.borrow().contains_key(&(runner.current_x, runner.current_y, runner.cheats)) {

                let length_to_goal = *self.cache.borrow().get(&(runner.current_x, runner.current_y, runner.cheats)).unwrap();

                for i in  0.. runner.already_visited_list.len() {
                    let v = runner.already_visited_list[i];
                    let length_to_goal = runner.already_visited_list.len() - i + length_to_goal;
                    self.cache.borrow_mut().insert(v, length_to_goal);
                }
                done_runners.push(length_to_goal + runner.already_visited_list.len());
                continue;
            }

            if runner.current_x == end.0 && runner.current_y == end.1 {
                //is done
                //println!("Done");
                //done_runners.push(runner);
                for i in  0.. runner.already_visited_list.len() {
                    let v = runner.already_visited_list[i];
                    let length_to_goal = runner.already_visited_list.len() - i + 1;
                    self.cache.borrow_mut().insert(v, length_to_goal);
                }
                done_runners.push(runner.already_visited_list.len());
                continue;
            }

            let mut should_cheat = false;
            if self.get(runner.current_x, runner.current_y) == '#' {
                should_cheat = runner.cheats > 0;
                if !should_cheat {
                    continue;
                }
            }
            for dir in directions {
                queue.push_back(runner.cmove(dir.0, dir.1, should_cheat));
            }
        }
        done_runners
    }
}

struct Runner {
    current_x: i64,
    current_y: i64,
    already_visited_list: Vec<(i64, i64, usize)>,
    already_visited: HashSet<(i64,i64)>,
    cheats: usize,
}

impl Runner {
    fn new(x: i64, y: i64, num_cheats: usize) -> Self {
        Runner {
            current_x: x,
            current_y: y,
            already_visited: HashSet::new(),
            cheats: num_cheats,
            already_visited_list: Vec::new(),
        }
    }
    fn cmove(&self, dx: i64, dy: i64, should_cheat: bool) -> Runner {
        let mut new_visited = self.already_visited.clone();
        new_visited.insert((self.current_x,self.current_y));
        let cheats_left = self.cheats - if should_cheat { 1 } else { 0 };
        let mut new_visited_list: Vec<(i64,i64, usize)> = Vec::new();
        new_visited_list = self.already_visited_list.clone();
        new_visited_list.push((self.current_x, self.current_y, self.cheats));
        Runner {
            cheats: cheats_left,
            current_x: self.current_x + dx,
            current_y: self.current_y + dy,
            already_visited: new_visited,
            already_visited_list: new_visited_list,
        }
    }

    pub(crate) fn has_already_visited_current(&self) -> bool {
        self.already_visited.contains(&(self.current_x, self.current_y))
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
    let mut smap = SMap {
        map, width: width as i64, height: height as i64,
        cache: RefCell::new(HashMap::new())
    };

    let paths_without_cheating = smap.find_paths(start_pos, end_pos, 0, 10000);
    let depth = paths_without_cheating[0];
    println!("no cheating: {depth}");

    let cheat_paths = smap.find_paths(start_pos, end_pos, 2, depth - 100);
    let count = cheat_paths.iter().filter(|l| **l <= depth - 100).count();
    println!("part 1: {:?}", count);
    //63966 too high
    //29849 too high
}