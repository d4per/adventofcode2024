

fn is_safe(line: &Vec<u64>) -> bool {
    let mut last_number: Option<i64> = Option::None;
    let mut direction: Option<i8> = None;
    for n in line {
        let is_ok =match (*n as i64, last_number, direction) {
            (nn, None, _) => {
                last_number = Some(nn);
                true
            },
            (nn, Some(latest), None) => {
                let step_size_ok = (1i64..=3).contains(&(nn - latest).abs());
                if !step_size_ok {
                    return false;
                }
                direction = Some((nn - latest).signum() as i8);
                last_number = Some(nn);
                true
            },

            (nn, Some(latest), Some(dir)) => {
                let step_size_ok = (1i64..=3).contains(&(nn - latest).abs());
                if !step_size_ok {
                    return false;
                }
                let new_direction = (nn - latest).signum() as i8;
                last_number = Some(nn);
                new_direction == dir
            },

        };
        if !is_ok {
            return false;
        }
    }
    true
}


fn is_safe_with_one_exception(line: &Vec<u64>) -> bool {
    if is_safe(line) {
        return true;
    }
    for i in 0 .. line.len() {
        let mut line_copy = line.clone();
        line_copy.remove(i);
        if is_safe(&line_copy) {
            return true;
        }
    }
    false
}
fn main() {
    let mut lines = Vec::<Vec<u64>>::new();

    include_str!("input.txt")
        .split("\n")
        .map(|line| {
            let s = line.split(" ");
            let v = Vec::from_iter(s.map(|number| number.parse::<u64>().unwrap()));
            v
        })
        .for_each(|v| lines.push(v));


    //part 1
    //let safe_lines: i64 = lines.iter().map(|line| if is_safe(line) { 1 }else { 0 }).sum();

    //part 2
    let safe_lines: i64 = lines.iter().map(|line| if is_safe_with_one_exception(line) { 1 }else { 0 }).sum();


    println!("safe_lines: {:?}", safe_lines);

}