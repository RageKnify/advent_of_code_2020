use std::fs;
use std::io;

fn part1(joltages: &[usize]) -> usize {
    let mut iter = joltages.iter();
    let mut last = &0;

    let mut ones = 0;
    let mut threes = 1;

    while let Some(curr) = iter.next() {
        let diff = curr - last;
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
        last = curr;
    }
    ones * threes
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut joltages: Vec<_> = file
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();

    joltages.sort();

    let res = part1(&joltages);
    println!("res = {}", res);
    Ok(())
}
