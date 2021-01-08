use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
enum Action {
    Mask(u64, u64),
    Write(usize, u64),
}

impl From<&str> for Action {
    fn from(string: &str) -> Self {
        if string.starts_with("mask") {
            let mask_str = string.rsplit(" = ").next().unwrap();
            let ones_str = mask_str
                .chars()
                .rev()
                .map(|c| if c == 'X' { '1' } else { c })
                .rev()
                .collect::<String>();
            let ones = u64::from_str_radix(&ones_str, 2).unwrap();
            let zeros_str = mask_str
                .chars()
                .rev()
                .map(|c| if c != 'X' { c } else { '0' })
                .rev()
                .collect::<String>();
            let zeros = u64::from_str_radix(&zeros_str, 2).unwrap();
            Action::Mask(ones, zeros)
        } else {
            let mut iter = string.split(" = ");
            let mut first = iter.next().unwrap().split(|c: char| c == '[' || c == ']');
            first.next();
            let idx = first
                .next()
                .map(|s| usize::from_str_radix(s, 10).unwrap())
                .unwrap();
            let val = iter
                .next()
                .map(|s| u64::from_str_radix(s, 10).unwrap())
                .unwrap();
            Action::Write(idx, val)
        }
    }
}

fn part1(actions: &[Action]) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut iter = actions.iter();
    let mut ones;
    let mut zeros;
    if let Action::Mask(o, z) = iter.next().unwrap() {
        ones = o;
        zeros = z;
        for action in iter {
            match action {
                Action::Write(idx, value) => {
                    let v = (value | zeros) & ones;
                    mem.insert(*idx, v);
                }
                Action::Mask(o, z) => {
                    ones = o;
                    zeros = z;
                }
            }
        }
    }
    mem.values().sum()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let actions: Vec<_> = file.lines().map(Action::from).collect();
    let res = part1(&actions);
    println!("res = {}", res);
    Ok(())
}
