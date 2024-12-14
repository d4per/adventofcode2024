use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use regex::Regex;



struct GardenMap {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    regions: Vec<Region>,


}

struct Region {
    region_points: HashSet<(i32, i32)>,
}

impl Region {

    fn area(&self) -> usize {
        self.region_points.len()
    }

    fn perimeter(&self) -> usize {
        let directions = vec![(-1,0), (1,0), (0,-1), (0,1)];
        let mut tot_perimeters = 0;
        for p in self.region_points.clone() {
            for d in directions.iter() {
                let n = (p.0 + d.0, p.1 + d.1);
                if self.region_points.contains(&n) {
                    //no perimeter this way
                } else {
                    tot_perimeters += 1;
                }
            }
        }

        tot_perimeters
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn price2(&self) -> usize {
        self.area() * self.sides()
    }

    fn sides(&self) -> usize {
        let mut vertical_map: HashSet<(i32, i32)> = HashSet::new();
        let mut horizontal_map: HashSet<(i32, i32)> = HashSet::new();

        let directions = vec![(-1,0, '|'), (1,0, '|'), (0,-1, '-'), (0,1, '-')];

        for p in self.region_points.clone() {
            for d in directions.iter() {
                let n = (p.0 + d.0, p.1 + d.1);
                if self.region_points.contains(&n) {
                    //no perimeter this way
                } else {
                    let dx = if d.0 <= 0 { 0 } else { 1 };
                    let dy = if d.1 <= 0 { 0 } else { 1 };
                    if d.0.abs() > 0 {
                        //horizontal
                        horizontal_map.insert((p.0 + dx, p.1 + dy));
                    } else {
                        //vertical
                        vertical_map.insert((p.0 + dx, p.1 + dy));
                    }
                }
            }
        }

        let mut tot_perimeters = 0;
        let max_hx = horizontal_map.iter().map(|(x,y)| x).max().unwrap();
        let max_hy = horizontal_map.iter().map(|(x,y)| y).max().unwrap();
        let mut found = false;
        for y in -2..=(*max_hy+2) {
            for x in -2..=(*max_hx+2) {
                let contains = horizontal_map.contains(&(x, y));
                match (contains, found) {
                    (false, false) => {},
                    (false, true) => {
                        found = false;
                        tot_perimeters += 1;
                    },
                    (true, _) => {
                        found = true;
                    }
                }
            }
        }


        let max_vx = vertical_map.iter().map(|(x,y)| x).max().unwrap();
        let max_vy = vertical_map.iter().map(|(x,y)| y).max().unwrap();
        for x in -2..=(*max_vx+2) {
            for y in -2..=(*max_vy+2) {
                let contains = vertical_map.contains(&(x, y));
                match (contains, found) {
                    (false, false) => {},
                    (false, true) => {
                        found = false;
                        tot_perimeters += 1;
                    },
                    (true, _) => {
                        found = true;
                    }
                }
            }
        }

        tot_perimeters
    }

}

impl GardenMap {

    fn new(map: &Vec<Vec<char>>) -> Self {
        GardenMap {
            map: map.clone(),
            height: map[0].len(),
            width: map[0].len(),
            regions: Vec::<Region>::new(),
        }
    }

    fn find_region(&self, start_point: &(i32,i32), c: char) -> Region {
        let directions = vec![(-1,0), (1,0), (0,-1), (0,1)];
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((start_point.0 as i32, start_point.1 as i32));
        //let mut already_found: HashSet<(usize, usize)> = HashSet::new();
        let mut region_points: HashSet<(i32, i32)> = HashSet::new();
        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            let (x,y) = p;
            if region_points.contains(&p)  || x < 0 || y < 0 || x>=self.width as i32 || y>= self.height as i32 || self.map[y as usize][x as usize] != c {
                continue;
            }
            region_points.insert(p);
            for (dx, dy) in &directions {
                queue.push_back((x + dx, y + dy));
            }
        }


        Region {
            region_points
        }
    }

    fn find_regions(&mut self) {
        let mut already_found: HashSet<(i32, i32)> = HashSet::new();


        for y in 0i32..self.height as i32 {
            for x in 0i32..self.width as i32 {
                let point = (x,y);
                if already_found.contains(&point) {
                    continue;
                }
                let c = self.map[y as usize][x as usize];
                let region = self.find_region(&point, c);
                already_found.extend(&region.region_points);
                self.regions.push(region);
                already_found.insert(point);
            }
        }

        println!("regions: {}", self.regions.len())
    }

    fn tot_price(&self) -> usize {

        self.regions.iter().map(|r| r.price()).sum()
    }

    fn tot_price2(&self) -> usize {
        self.regions.iter().map(|r| r.price2()).sum()
    }

}


fn main() {

    let map: Vec<Vec<char>> = include_str!("test.txt")
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();


    let mut garden_map = GardenMap::new(&map);

    garden_map.find_regions();

    let tot_price = garden_map.tot_price();
    println!("part 1: {:?}", tot_price);

    let tot_price2 = garden_map.tot_price2();
    println!("part 2: {:?}", tot_price2);
    //1077954 too high
}