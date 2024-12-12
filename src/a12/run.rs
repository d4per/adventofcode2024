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

    fn sides(&self) -> usize {
        let mut map: Vec<Vec<char>> = Vec::new();
        let max_x = self.region_points.iter().map(|p| p.0).max().unwrap();
        let max_y = self.region_points.iter().map(|p| p.1).max().unwrap();
        for y in 0..=max_y {
            let mut v = Vec::new();
            for x in 0..=max_x {
                v.push('.');
            }
            map.push(v);
        }


        let directions = vec![(-1,0, '|'), (1,0, '|'), (0,-1, '-'), (0,1, '-')];
        let mut tot_perimeters = 0;
        for p in self.region_points.clone() {
            for d in directions.iter() {
                let n = (p.0 + d.0, p.1 + d.1);
                if self.region_points.contains(&n) {
                    //no perimeter this way
                } else {
                    map[n.1][n.0] = d.2;
                }
            }
        }
        todo!()


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

}


fn main() {

    let map: Vec<Vec<char>> = include_str!("input.txt")
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();


    let mut garden_map = GardenMap::new(&map);

    garden_map.find_regions();

    let tot_price = garden_map.tot_price();
    println!("{:?}", tot_price);



}