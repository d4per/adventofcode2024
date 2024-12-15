use std::cell::{Cell, RefCell};
use regex::Regex;

#[derive(Debug, Clone)]
struct SMap {
    map: RefCell<Vec<char>>,
    width: usize,
    height: usize
}
impl SMap {

    fn get(&self, x: usize, y: usize) -> char {
        self.map.borrow()[x + y *self.width]
    }

    fn set(&self, x: usize, y: usize, c: char) {
        self.map.borrow_mut()[x + y *self.width]=c;
    }

    fn find_initial_pos(&self) -> (usize, usize) {
        let position = self.map.borrow().iter().enumerate().find(|(i, c)| **c == '@').map(|(i, c)| i).unwrap();
        (position % self.width, position / self.width)
    }

    fn move_square(&self, from: (usize, usize), to: (usize, usize)) {
        let c = self.get(from.0, from.1);
        self.set(to.0, to.1, c);
        self.set(from.0, from.1, '.');

    }

    fn try_move_to(&self, direction: (i8, i8), robot_pos: (usize, usize)) -> bool {
        let (px, py) = ((robot_pos.0 as i32 + direction.0 as i32) as usize, (robot_pos.1 as i32 + direction.1 as i32) as usize);
        let pc = self.get(px, py);
        match pc {
            '#' => {
                //no move possible
                false
            },
            'O' => {
                //recursive move
                if self.try_move_to(direction, (px, py)) {
                    self.move_square(robot_pos, (px, py));
                    true
                } else {
                    false
                }
            },

            _ => {
                //move possible
                self.move_square(robot_pos, (px, py));
                true
            }
        }
    }

    fn gps_pos(x: usize, y: usize) -> usize {
        x + y * 100
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x,y);
                print!("{c}");
            }
            println!();
        }
    }

    fn num_o(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x,y);
                if c == 'O' {
                    count += 1;
                }
            }
        }
        count
    }
}


fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));

    let mut raw_map: Vec<Vec<char>> = lines.iter().take_while(|line| !line.is_empty()).map(|line| line.chars().collect()).collect();
    let width = raw_map[0].len();
    let height = raw_map.len();
    let flat_map: Vec<char> = raw_map.into_iter().flatten().collect();
    let map = SMap {
        map: RefCell::new(flat_map),
        width, height
    };

    let directions: Vec<char> = lines.into_iter().filter(|line| !(line.is_empty() || line.contains("#"))).map(|line| line.chars()).flatten().collect();

    let mut robot_pos = map.find_initial_pos();

    for d in directions {
        let num_o = map.num_o();
        //println!("---------------");
        //println!("o-count: {num_o}");
        //println!("direction: {d}");
        //map.print();
        let (x, y) = robot_pos;
        let (d, should_move) = match d {
            '<' => {
                let dir = (-1, 0);
                let result = map.try_move_to(dir, (x,y));
                (dir, result)
            },
            '>' => {
                let dir = (1, 0);
                let result = map.try_move_to(dir, (x,y));
                (dir, result)
            },
            'v' => {
                let dir = (0, 1);
                let result = map.try_move_to(dir, (x,y));
                (dir, result)
            },
            '^' => {
                let dir = (0, -1);
                let result = map.try_move_to(dir, (x,y));
                (dir, result)
            },
            _ => {panic!("unknown direction")},
        };

        if should_move {
            robot_pos = ((x as i32 + d.0 as i32) as usize, (y as i32 + d.1 as i32) as usize);
        }
    }
    map.print();

    let mut sum_gps = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 'O' {
                sum_gps += SMap::gps_pos(x,y);
            }
        }
    }

    println!("part 1: sum_gps = {sum_gps:?}");
    // 1512818 too low




}