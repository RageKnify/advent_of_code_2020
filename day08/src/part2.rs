use std::fs;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
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
            Some("nop") => Ok(Self::NOP(num)),
            _ => panic!(),
        }
    }
}

fn run(instructions: &[Instruction]) -> Result<i32, Vec<usize>> {
    let mut visited = Vec::<usize>::new();
    let mut current = 0;
    let mut acc = 0;
    while current < instructions.len() {
        if visited.contains(&current.clone()) {
            visited.sort();
            return Err(visited);
        }
        visited.push(current);
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
            Instruction::NOP(_) => {
                current += 1;
            }
        }
    }
    Ok(acc)
}

fn find_last_nop_or_jmp(
    instructions: &[Instruction],
    visited: &[usize],
    last_relevant: usize,
) -> usize {
    for instruction_idx in visited.iter().rev() {
        if *instruction_idx >= last_relevant {
            continue;
        }
        match instructions[*instruction_idx] {
            Instruction::ACC(_) => continue,
            _ => {
                return *instruction_idx;
            }
        }
    }
    unreachable!()
}

fn switch(instructions: &mut [Instruction], idx: usize) {
    let instruction = instructions.get_mut(idx).unwrap();
    match instruction {
        Instruction::ACC(_) => unreachable!(),
        Instruction::JMP(n) => *instruction = Instruction::NOP(*n),
        Instruction::NOP(n) => *instruction = Instruction::JMP(*n),
    };
}

fn solve(instructions: Vec<Instruction>) -> i32 {
    match run(&instructions) {
        Ok(res) => res,
        Err(out_visited) => {
            let mut visited = out_visited;
            let mut last_relevant = instructions.len() - 1;
            loop {
                let mut current = instructions.clone();
                last_relevant = find_last_nop_or_jmp(&instructions, &visited, last_relevant);
                switch(&mut current, last_relevant);
                match run(&current) {
                    Ok(res) => break res,
                    Err(in_visited) => visited = in_visited,
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let instructions: Vec<Instruction> = file.lines().flat_map(Instruction::from_str).collect();
    let res = solve(instructions);
    println!("res = {}", res);
    Ok(())
}
