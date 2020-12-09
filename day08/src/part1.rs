use std::collections::HashSet;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let variant = parts.next();
        let num = i32::from_str_radix(parts.next().unwrap(), 10).unwrap();
        match variant {
            Some("acc") => Ok(Self::ACC(num)),
            Some("jmp") => Ok(Self::JMP(num)),
            Some("nop") => Ok(Self::NOP),
            _ => panic!(),
        }
    }
}

fn run(instructions: &[Instruction]) -> i32 {
    let mut visited = HashSet::<usize>::new();
    let mut current = 0;
    let mut acc = 0;
    while !visited.contains(&current.clone()) {
        visited.insert(current);
        let instruction = instructions[current];
        match instruction {
            Instruction::ACC(increment) => {
                acc += increment;
                current += 1
            }
            Instruction::JMP(delta) => {
                if delta > 0 {
                    current += delta as usize;
                } else {
                    current -= -delta as usize;
                }
            }
            Instruction::NOP => {
                current += 1;
            }
        }
    }
    acc
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let instructions: Vec<Instruction> = file.lines().flat_map(Instruction::from_str).collect();
    let res = run(&instructions);
    println!("res = {}", res);
    Ok(())
}
