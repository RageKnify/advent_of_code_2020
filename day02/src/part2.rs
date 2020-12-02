use std::fs;
use std::io;

fn valid<'r>(line: &'r &str) -> bool {
    let mut parts = line.split(' ');
    let (first_idx, last_idx) = {
        let mut numbers = parts.next().unwrap().split('-');
        let first_n = usize::from_str_radix(numbers.next().unwrap(), 10).unwrap();
        let second_n = usize::from_str_radix(numbers.next().unwrap(), 10).unwrap();
        (first_n - 1, second_n - 1)
    };
    let chr = parts.next().unwrap().chars().next().unwrap();
    let password = parts.next().unwrap();
    let mut password_chars = password.chars();
    let first = password_chars
        .nth(first_idx)
        .map(|c| c == chr)
        .unwrap_or(false);
    let last = password_chars
        .nth(last_idx - first_idx - 1)
        .map(|c| c == chr)
        .unwrap_or(false);
    first ^ last
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let sum = file.lines().filter(valid).count();
    println!("sum = {}", sum);
    Ok(())
}
