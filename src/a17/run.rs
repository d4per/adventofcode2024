use std::cell::{Cell, RefCell};
use std::ops::{BitAnd, BitXor, Shl};
use itertools::Itertools;
use num_bigint::BigUint;
use regex::Regex;
use num_traits::cast::ToPrimitive;
use num_traits::Zero;
struct Registers {
    a: RefCell<BigUint>,
    b: RefCell<BigUint>,
    c: RefCell<BigUint>,
    pc: Cell<usize>,
    output: Vec<u128>
}

fn main() {

    //test
    // Register A: 729
    // Register B: 0
    // Register C: 0
    //
    // Program: 0,1,5,4,3,0

    //Register A: 22571680
    //Register B: 0
    //Register C: 0



    let program_str = "2,4,1,3,7,5,0,3,4,3,1,5,5,5,3,0";
    let ai = 22571680u64;
    let bi = 0u64;
    let ci = 0u64;

    // let program_str = "0,1,5,4,3,0";
    // let ai = 729u64;
    // let bi = 0u64;
    // let ci = 0u64;
    let mut old_num = 0i64;

    for ai in 0..10000000000u64 {
        if ai % 10000000 == 0 {
            println!("{ai}");
        }

        let program: Vec<u8> = program_str.split(",").map(|c| c.parse::<u8>().unwrap()).collect();

        let registers = Registers {
            a: RefCell::new(BigUint::from(ai)),
            b: RefCell::new(BigUint::from(bi)),
            c: RefCell::new(BigUint::from(ci)),
            pc: Cell::new(0),
            output: Vec::new(),
        };

        let mut out: Vec<u8> = Vec::new();
        loop {
            let pc = registers.pc.get();
            if pc >= program.len() {
                break;
            }
            let instruction = program[pc as usize];
            let operand = program[(pc + 1) as usize];

            let combo = match operand {
                n if n >= 0 && n <= 3 => { BigUint::from(n) },
                4 => registers.a.borrow().clone(),
                5 => registers.b.borrow().clone(),
                6 => registers.c.borrow().clone(),
                _ => { panic!("unknown operand") }
            };

            match instruction {
                0 => {
                    //adv
                    let numerator = registers.a.borrow().clone();
                    let denominator = BigUint::from(1u64).shl(combo.to_u64().unwrap() as i64);
                    let result = numerator / denominator;
                    registers.a.replace(result);
                    registers.pc.set(registers.pc.get() + 2);
                },
                1 => {
                    //bxl
                    let b = registers.b.borrow().clone();
                    registers.b.replace(b.bitxor(BigUint::from(operand)));
                    registers.pc.set(registers.pc.get() + 2);
                },
                2 => {
                    //bst
                    registers.b.replace(BigUint::from(combo).bitand(BigUint::from(7u64)));
                    registers.pc.set(registers.pc.get() + 2);
                },
                3 => {
                    //jnz
                    if !registers.a.borrow().clone().is_zero() {
                        registers.pc.set(operand as usize);
                    } else {
                        registers.pc.set(registers.pc.get() + 2);
                    }
                },
                4 => {
                    //bxc
                    let b = registers.b.borrow().clone();
                    registers.b.replace(b.bitxor(registers.c.borrow().clone()));
                    registers.pc.set(registers.pc.get() + 2);
                },
                5 => {
                    //out
                    out.push(combo.bitand(BigUint::from(7u64)).to_u8().unwrap());
                    registers.pc.set(registers.pc.get() + 2);
                    if out.len()> 9 {
                        //out too long
                        break;
                    }
                },
                6 => {
                    //bdv
                    let numerator = registers.a.borrow().clone();
                    let denominator = BigUint::from(1u64).shl(combo.to_u64().unwrap());
                    let result = numerator / denominator;
                    registers.b.replace(result);
                    registers.pc.set(registers.pc.get() + 2);
                },
                7 => {
                    //bdv
                    let numerator = registers.a.borrow().clone();
                    let denominator = BigUint::from(1u64).shl(combo.to_u64().unwrap());
                    let result = numerator / denominator;
                    registers.c.replace(result);
                    registers.pc.set(registers.pc.get() + 2);
                },
                _ => {
                    panic!();
                }
            }



        }

        let out_str = out.clone().into_iter().join(",");
        let out_num: usize = out.clone().into_iter().rev().join("").parse().unwrap();
        let out_delta = out_num as i64 - old_num;
        println!("{ai} {out_num} {out_delta}");
        old_num = out_num as i64;
        if program == out {
            println!("part 2: {ai}");
            break;
        }

        //let out_str = out.clone().into_iter().join(",");
        //println!("part 1: {out_str}");
        //6,2,4,3,0,6,0,4,7
        //2,0,1,3,4,0,2,1,7
    }
}