use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Square {
    c: char,
    x: usize,
    y: usize,
    antinode: RefCell<bool>,
    dist_map: HashMap<char, u64>,
}

impl Square {
    fn new(c: char, x: usize, y: usize) -> Square {
        Self {
            c,
            x,
            y,
            antinode: RefCell::new(false),
            dist_map: HashMap::new()
        }
    }
    
    fn set_antinode(&self, is_antinode: bool) {
        self.antinode.replace(is_antinode);
    }
}

#[derive(Debug)]
struct SMap {
    width: usize,
    height: usize,
    squares: Vec<Rc<Square>>,
}

impl SMap {
    fn new(width: usize, height: usize) -> SMap {
        Self {
            width, height,
            squares: Vec::new(),
        }
    }

    fn mark_antinode(&mut self, coord: (i64, i64)) {
        let (x,y) = coord;
        if x < 0 || y < 0 || x>= self.width as i64 || y>= self.height as i64 {
            return;
        }
        self.squares[(x+ y * self.width as i64) as usize].set_antinode(true);
    }
    
    fn count_antinodes(&self) -> usize {
        self.squares.iter().filter(|s| s.antinode.take()).count()
    }
    
    // fn add(&mut self, square: Square) -> &Square {
    //     self.squares.push(square);
    //     &self.squares[self.squares.len() - 1]
    // }
}

fn main() {

    let map: Vec<Vec<char>> = include_str!("input.txt")
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let height = map.len();
    let width = map[0].len();
    let mut smap = SMap::new(width, height);

    let mut squares_with_antennas: Vec<Rc<Square>> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let c = map[y][x];
            let square = Rc::new(Square::new(c, x , y));
            smap.squares.push(square.clone());
            squares_with_antennas.push(square);
        }
    }
    println!("{:?}", &smap);

    let squares_by_antenna = squares_with_antennas.iter().into_group_map_by(|e| e.c);

    for (c, e) in squares_by_antenna.iter() {
        match c {
            '.' => {},
            frequency => {
                for s1 in e {
                    for s2 in e {
                        if !s1.eq(s2) {
                            //mark_antinodes(&s1, &s2, &mut smap);  part 1
                            mark_antinode_line(&s1, &s2, &mut smap);  //part 2
                        }
                    }
                }
            }
        }

    }

    print!("Num antinodes: {}", smap.count_antinodes());
    
    
}

fn mark_antinodes(s1: &&&Rc<Square>, s2: &&&Rc<Square>, map: &mut SMap) {
    println!("mark antinode {} {}", s1.x, s1.y);
    let dx = s1.x as i64 - s2.x as i64;
    let dy = s1.y as i64 - s2.y as i64;
    let new_p1 = (s1.x as i64 + dx, s1.y as i64 + dy);
    let new_p2 = (s2.x as i64 - dx, s2.y as i64 - dy);
    map.mark_antinode(new_p1);
    map.mark_antinode(new_p2);
}

fn mark_antinode_line(s1: &&&Rc<Square>, s2: &&&Rc<Square>, map: &mut SMap) {
    println!("mark antinode {} {}", s1.x, s1.y);
    let dx = s1.x as i64 - s2.x as i64;
    let dy = s1.y as i64 - s2.y as i64;
    for i in -100..100 {
        let new_p = (s1.x as i64 + dx * i, s1.y as i64 + dy * i);
        map.mark_antinode(new_p);
    }
}