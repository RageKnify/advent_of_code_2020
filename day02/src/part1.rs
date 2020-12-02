use std::fs;
use std::io;

fn valid<'r>(line: &'r &str) -> bool {
    let mut parts = line.split(' ');
    let amounts = parts.next().unwrap();
    let (min, max) = {
        let mut numbers = amounts.split('-');
        let min = usize::from_str_radix(numbers.next().unwrap(), 10).unwrap();
        let max = usize::from_str_radix(numbers.next().unwrap(), 10).unwrap();
        (min, max)
    };
    let chr = parts.next().unwrap().chars().next().unwrap();
    let password = parts.next().unwrap();
    let count = password.chars().filter(|c| *c == chr).count();
    min <= count && count <= max
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let sum = file.lines().filter(valid).count();
    println!("sum = {}", sum);
    Ok(())
}
