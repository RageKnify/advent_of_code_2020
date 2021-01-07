use std::fs;
use std::io;

fn part1(start: usize, bus_ids: &[usize]) -> usize {
    let mut best_time = usize::MAX;
    let mut best_result = 0;
    for id in bus_ids {
        let base = start / id;
        let mut new_time = base * id;
        if new_time < start {
            new_time = (base + 1) * id;
        }
        if new_time < best_time {
            best_time = new_time;
            best_result = id * (best_time - start);
        }
    }
    best_result
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut lines = file.lines();
    let start = lines
        .next()
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .unwrap();
    let bus_ids: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| usize::from_str_radix(s, 10))
        .flatten()
        .collect();
    let res = part1(start, &bus_ids);
    println!("res = {}", res);
    Ok(())
}
