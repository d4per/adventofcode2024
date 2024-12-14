
use regex::Regex;

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Game {
    fn new() -> Self {
        Game {
            a: (0, 0),
            b: (0, 0),
            prize: (0, 0)
        }
    }
}

fn main() {
    let mut lines = Vec::<&str>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| lines.push(line));


    let a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let p_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let re = Regex::new(r"\w\w\w(?<name>\w+)").unwrap();

    let mut games:Vec<Game> = Vec::new();
    let mut game = Game::new();
    for line in lines {
        if line.is_empty() {
            games.push(game);
            game = Game::new();
        } else {
            let a = a_re.captures(line);
            let b = b_re.captures(line);
            let p = p_re.captures(line);

            a.iter().for_each(|aa| game.a = (aa.get(1).unwrap().as_str().parse::<i64>().unwrap(), aa.get(2).unwrap().as_str().parse::<i64>().unwrap()));
            b.iter().for_each(|aa| game.b = (aa.get(1).unwrap().as_str().parse::<i64>().unwrap(), aa.get(2).unwrap().as_str().parse::<i64>().unwrap()));
            p.iter().for_each(|aa| game.prize = (aa.get(1).unwrap().as_str().parse::<i64>().unwrap(), aa.get(2).unwrap().as_str().parse::<i64>().unwrap()));
        }
    }
    games.push(game);

    println!("{:?}", &games);

    // P = n1 * A + n2 * B
    // Px = n1 * Ax + n2 *Bx
    // Py = n1 * Ay + n2 *By
    // n1 = (Px - n2 * Bx) / Ax
    // (Px - n2 * Bx) / Ax = (Py - n2 * By) / Ay
    // (Px - n2 * Bx) * Ay = (Py - n2 * By) * Ax
    // Px * Ay - n2 * Bx * Ay = Py * Ax - n2 * By * Ax
    // n2 * By * Ax - n2 * Bx *Ay = Py * Ax - Px * Ay
    // n2 = (Py * Ax - Px * Ay) / (By * Ax - Bx * Ay)

    let mut tot_cost = 0;
    for game in games {
        let (a_x, a_y) = game.a;
        let (b_x, b_y) = game.b;
        //let (p_x, p_y) = game.prize; //part 1
        let (p_x, p_y) = (game.prize.0 + 10000000000000i64, game.prize.1  + 10000000000000i64);  //part 2


        let debug = (p_y as i128 * a_x as i128 - p_x as i128 * a_y as i128);
        println!("debug {debug}  {}", (debug as f64));
        let n2 = (p_y as i128 * a_x as i128 - p_x as i128 * a_y as i128) as f64 / (b_y * a_x - b_x * a_y) as f64;
        let n1 = (p_x as i128 - n2 as i128 * b_x as i128) as f64 / a_x as f64;

        let nn2 = n2.round();
        let nn1 = n1.round();

        if nn1 >= 0.0 && nn2 >= 0.0 && (n1 - nn1).abs() < 0.01 && (n2 - nn2).abs() < 0.01 {
            //winning game
            let cost = nn1 as u64 * 3 + nn2 as u64 * 1;
            tot_cost += cost;
        } else {
            //no prize
            println!("no prize");
        }
    }

    println!("part cost= {}", tot_cost);
    //28663
    //28887
    //616807771018285657
    //97535138175685 too high
    //96979582619758


}