use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
enum Action {
    Mask(String),
    Write(usize, u64),
}

impl From<&str> for Action {
    fn from(string: &str) -> Self {
        if string.starts_with("mask") {
            let mask_str = string.rsplit(" = ").next().unwrap();
            Action::Mask(mask_str.into())
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

fn gen_idxs(start: usize, mask: &str) -> Vec<usize> {
    fn gen_next(mut curr: usize, mut xs: usize) -> usize {
        while xs != 0 {
            let lowest_bit = xs.trailing_zeros();
            let mask = 1usize << lowest_bit;
            xs ^= mask;
            if 0 == (curr & mask) {
                return curr | mask;
            }
            curr ^= mask;
        }
        unreachable!()
    }

    let mut res = Vec::new();
    let ones = usize::from_str_radix(
        &mask
            .chars()
            .map(|c| if c == '1' { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let xs = usize::from_str_radix(
        &mask
            .chars()
            .map(|c| if c == 'X' { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap();
    // ones start enabled
    // xs start at 0
    let start = (start | ones) & !xs;
    let end = start | xs;
    let mut current = start;
    while current < end {
        res.push(current);
        current = gen_next(current, xs);
    }
    res.push(current);
    res
}

fn part2(actions: &[Action]) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut iter = actions.iter();
    let mut mask;
    if let Action::Mask(m) = iter.next().unwrap() {
        mask = m;
        for action in iter {
            match action {
                Action::Write(idx, value) => {
                    let idxs = gen_idxs(*idx, mask.as_ref());
                    for idx in idxs {
                        mem.insert(idx, *value);
                    }
                }
                Action::Mask(m) => {
                    mask = m;
                }
            }
        }
    }
    mem.values().sum()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let actions: Vec<_> = file.lines().map(Action::from).collect();
    let res = part2(&actions);
    println!("res = {}", res);
    Ok(())
}
