use std::cell::Cell;
use std::cmp::max;
use std::collections::HashSet;
use regex::Regex;


#[derive(Debug, Clone)]
struct Robot {
    current_x: Cell<i64>,
    current_y: Cell<i64>,
    v_x: i64,
    v_y: i64,


}

impl Robot {

    fn run_timesteps(&self, map_width: i64, map_height: i64, time_steps: i64) {
        let new_x = self.current_x.get() + self.v_x * time_steps;
        let new_y = self.current_y.get() + self.v_y * time_steps;

        self.current_x.set((new_x + map_width * time_steps) % map_width);
        self.current_y.set((new_y + map_height * time_steps) % map_height);
    }
}

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));


    let re = Regex::new(r"p=(\d+),(\d+) v=((-|\d+)+),((-|\d)+)").unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    for line in lines {
        let capture = re.captures(line).unwrap();
        let start_x = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let start_y = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let v_x = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let v_y = capture.get(5).unwrap().as_str().parse::<i64>().unwrap();
        let robot = Robot {
            current_x:Cell::new(start_x), current_y:Cell::new(start_y), v_x, v_y
        };
        robots.push(robot);
    }

    let map_width = 101;
    let map_height = 103;

    let part2_robots = robots.clone();

    for robot in robots.iter() {
        println!("robot {:?}", &robot);
        robot.run_timesteps(map_width, map_height, 100);

    }
    //display_map(&robots, map_width, map_height);

    let q1 = count_quadrant(&robots, map_width, map_height, 0,0);
    let q2 = count_quadrant(&robots, map_width, map_height, 1,0);
    let q3 = count_quadrant(&robots, map_width, map_height, 0,1);
    let q4 = count_quadrant(&robots, map_width, map_height, 1,1);

    let sum = q1 * q2 * q3 * q4;
    println!("part 1: sum= {sum}");
    //489 too low
    //220971520


    let r = part2_robots.clone();
    for i in 0..1000000000 {
        for robot in r.iter() {
            robot.run_timesteps(map_width, map_height, 1);
        }
        if is_christmastree2(&r, map_width, map_height) {
            display_map(&r, map_width, map_height);
            println!("num seconds: {i+1}");
            //6354 to low
            //6355
        }
    }


}

fn is_christmastree(robots: &Vec<Robot>, w: i64, h: i64) -> bool {
    let mut y_values: HashSet<i64> = HashSet::new();
    for robot in robots.iter()  {
        y_values.insert(robot.current_y.get());
    }
    let mut y_in_row = 0;
    let mut found = false;
    let mut max_y_in_row = 0;
    for y in 0..h {
        match (found, y_values.contains(&y)) {
            (true, true) => {
                y_in_row += 1;
            }
            (true, false) => {
                found = false;
                max_y_in_row = max_y_in_row.max(y_in_row);
                y_in_row = 0;
            },
            (false, true) => {
                found = true;
                y_in_row = 1;
            },
            (false, false) => {
            }

        }
    }
    max_y_in_row > (h - 2)
}

fn is_christmastree2(robots: &Vec<Robot>, w: i64, h: i64) -> bool {
    let mut coordinates: Vec<(i64, i64)> = Vec::new();
    for robot in robots.iter() {
        coordinates.push((robot.current_x.get(), robot.current_y.get()));
    }

    let mut num_neighbours = 0;
    for c1 in coordinates.iter() {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if !(dx == 0 && dy == 0) {
                    if coordinates.contains(&((*c1).0 + dx, (*c1).1 + dy)) {
                        num_neighbours += 1;
                    }
                }
            }
        }
    }

    //println!("{}",num_neighbours);
    num_neighbours > 500
}

fn display_map(robots: &Vec<Robot>, w: i64, h: i64) {
    for y in 0..h {
        for x in 0..w {
            let mut found = false;
            for robot in robots {
                if robot.current_x.get() == x && robot.current_y.get() == y {
                    found = true;
                }
            }
            if found {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn count_quadrant(robots: &Vec<Robot>, w: i64, h: i64, qx: i64, qy: i64) -> usize {
    let w2 = w /2;
    let h2 = h / 2;
    let start_x = w2 * qx + qx;
    let start_y = h2 * qy + qy;
    let mut count = 0;
    for y in start_y..(start_y + h2) {
        for x in start_x..(start_x + w2) {
            for robot in robots.iter() {
                if robot.current_x.get() == x && robot.current_y.get() == y {
                    count +=1;
                }
            }
        }
    }
    count
}