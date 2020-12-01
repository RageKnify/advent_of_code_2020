use std::fs;
use std::io;

const TARGET: u32 = 2020;

use day01::part1;

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut nums: Vec<u32> = file
        .lines()
        .map(|l| u32::from_str_radix(l, 10).unwrap())
        .collect();
    nums.sort();
    let (left, right) = part1(nums.as_slice(), TARGET).unwrap();
    println!("{} * {} = {}", left, right, left * right);
    return Ok(());
}
