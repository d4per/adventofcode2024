
use regex::Regex;
use std::mem;

#[derive(Debug)]
enum Operator {
    Plus(char),
    Mul(char),
    Concat(char),
}
impl Operator {
    fn variants() -> usize {
        3
    }
    
    fn from(operator_num: u64) -> Operator {
        match operator_num {
            0 => Operator::Plus('+'),
            1 => Operator::Mul('*'),
            2 => Operator::Concat('|'),
            _ => panic!("not implemented"),
            
        }
    }
    
    fn eval(&self, a: u64, b: u64) -> u64 {
        //println!("Eval {:?}", &self);
        match self {
            Operator::Plus(_) => { a + b}
            Operator::Mul(_) => { a * b},
            &Operator::Concat(_) => { format!("{}{}",a.to_string(), b.to_string()).parse::<u64>().unwrap() }
        }
    }
}

#[derive(Debug)]
struct Operators {
    oper_list: Vec<Operator>,


}

impl Operators {
    fn get_operator(&self, index: usize) -> &Operator {
        &self.oper_list[index]
    }
}

#[derive(Debug)]
struct Expression {
    result: u64,
    numbers: Vec<u64>,
}

impl Expression {
    fn new(line: String) -> Self {
        let parts = line.split(" ");
        let v: Vec<&str> = parts.collect();
        let result: u64 = v[0].split(":").next().unwrap().parse().unwrap();
        let numbers: Vec<u64> = v[1..].iter().map(|n| n.parse().unwrap()).collect();

        Expression {
            result,
            numbers
        }
    }

    fn num_permutations(&self) -> u64 {
        let num_operators = &self.numbers.len() - 1;
        ((Operator::variants()) as u64).pow(num_operators as u32)
    }
    
    fn generate_operators(&self, mut index: u64) -> Operators {
        let num_operators = &self.numbers.len() - 1;
        let variants = ((Operator::variants()) as u64);
        let mut oper_list: Vec<Operator> = Vec::new();


        for i in 0..num_operators {
            let operator = index % variants; 
            oper_list.push(Operator::from(operator));
            index = index / variants;
        }
        
        Operators {
            oper_list
        }
    }
    
    fn eval(&self, operators: &Operators) -> u64 {
        let mut accumulator= self.numbers[0];
        for i in 1..self.numbers.len() {
            let next_number = self.numbers[i];
            let next_operator = operators.get_operator(i - 1);
            accumulator = next_operator.eval(accumulator, next_number)  
        }
        
        accumulator
    }
}


fn main() {

    let expressions: Vec<Expression> = include_str!("input.txt")
        .split("\n")
        .map(|line| Expression::new(line.to_string()))
        .collect();

    //println!("capture: {:?}", expressions);

    let mut tot_result = 0;
    expressions.iter().for_each(|e| {
        let permutations = e.num_permutations();
        println!("permutations {}", permutations);
        for i in 0..permutations {
            let operators = e.generate_operators(i);
            //print!("{:?}", operators);
            let result = e.eval(&operators);
            if result == e.result {
                println!("{:?} {:?}", operators, e);
                tot_result+=result;
                break;
            } else {
                //println!("fail {:?} {:?} {}", operators, e, result);
            }
        }
    });

    println!("tot_result: {}", tot_result);
    // 896 too low
    //17732560 too low
}