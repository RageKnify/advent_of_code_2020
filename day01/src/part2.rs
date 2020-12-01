use std::fs;
use std::io;

const TARGET: u32 = 2020;

use day01::part1;

fn part2(nums: &[u32], value: u32) -> Option<(u32, u32, u32)> {
    let mut iter = nums.iter();
    while let Some(left) = iter.next() {
        if let Some((mid, right)) = part1(iter.as_slice(), value - left) {
            return Some((*left, mid, right));
        }
    }
    None
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut nums: Vec<u32> = file
        .lines()
        .map(|l| u32::from_str_radix(l, 10).unwrap())
        .collect();
    nums.sort();
    let (left, mid, right) = part2(nums.as_slice(), TARGET).unwrap();
    println!("{} * {} * {} = {}", left, mid, right, left * mid * right);
    Ok(())
}
