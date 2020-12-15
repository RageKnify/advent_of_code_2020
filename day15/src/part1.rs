use std::collections::HashMap;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let numbers: Vec<usize> = file
        .trim()
        .split(',')
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect();
    let mut last_idxs: HashMap<usize, usize> = HashMap::new();
    let mut last = 0;
    for idx in 0..2020 {
        let next;
        if idx < numbers.len() {
            if idx > 0 {
                last_idxs.insert(last, idx - 1);
            }
            next = numbers[idx];
        } else {
            next = last_idxs
                .insert(last, idx - 1)
                .map(|old| idx - 1 - old)
                .unwrap_or(0);
        }
        last = next;
    }
    println!("res = {}", last);
    Ok(())
}
