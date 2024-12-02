use counter::Counter;

fn part1() {
    
    let mut list1 = Vec::<u64>::new();
    let mut list2 = Vec::<u64>::new();
    
    include_str!("input.txt")
        .split("\n")
        .for_each(|line| {
            let mut split = line.split(" ");
            let first = split.next();
            let mut skip_split = split.skip_while(|token| *token == "" );
            let second = skip_split.next();
            //println!("{:?} {:?}", first, second);
            
            list1.push(first.unwrap().parse::<u64>().unwrap());
            list2.push(second.unwrap().parse::<u64>().unwrap());
        });

    list1.sort();
    list2.sort();
    
    //println!("list1 {:?}",list1);
    //println!("list2 {:?}",list1);
    
    let mut sumdiff = 0u64;
    for i in 0..list1.len() {
        let diff = ((list1[i] as i64) - (list2[i] as i64)).abs();
        sumdiff += diff as u64;
    }
    println!("total diff: {}", sumdiff)

}

fn main() {
    let mut list1 = Vec::<u64>::new();
    let mut list2 = Vec::<u64>::new();

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| {
            let mut split = line.split(" ");
            let first = split.next();
            let mut skip_split = split.skip_while(|token| *token == "" );
            let second = skip_split.next();
            //println!("{:?} {:?}", first, second);

            list1.push(first.unwrap().parse::<u64>().unwrap());
            list2.push(second.unwrap().parse::<u64>().unwrap());

        });

    let counts_counts = list2.iter().collect::<Counter<_>>();
    
    let mut tot_sum = 0u64;
    for i in 0..list1.len() {
        let elem1 = list1[i];
        let frequency_count = counts_counts.get(&elem1);
        let row_score = match frequency_count {
            None => 0u64,
            Some(count) => elem1 * (*count as u64),
        };
        tot_sum += row_score;
        
    }
    
    println!("tot score: {}", tot_sum);
}