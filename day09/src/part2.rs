use std::cmp::Ordering;
use std::convert::TryInto;
use std::fs;
use std::io;

fn is_bad(nums: &[usize; 25], target: usize) -> bool {
    for (idx, left) in nums.iter().enumerate() {
        for right in &nums[idx..] {
            if left + right == target {
                return false;
            }
        }
    }
    true
}

fn find_bad(nums: &[usize]) -> usize {
    let mut i = 0;
    loop {
        let curr = nums[i + 25];
        if is_bad((nums[i..i + 25]).try_into().unwrap(), curr) {
            return curr;
        }
        i += 1;
    }
}

fn find_sequence(nums: &[usize], bad: usize) -> (usize, usize) {
    let mut i = 0;
    let mut j = i + 1;
    let mut current_sum = nums[i] + nums[j];
    while i < nums.len() - 1 {
        match current_sum.cmp(&bad) {
            Ordering::Greater => {
                current_sum -= nums[i];
                i += 1;
            }
            Ordering::Equal => {
                return (i, j);
            }
            Ordering::Less => {
                j += 1;
                current_sum += nums[j];
            }
        }
    }
    unreachable!()
}

fn sum_bads(sequence: &[usize]) -> usize {
    // find lowest and highest then sum
    let min = sequence.iter().min().unwrap();
    let max = sequence.iter().max().unwrap();
    min + max
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let nums: Vec<_> = file
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();
    let bad = find_bad(&nums);
    println!("part1 = {}", bad);
    let (start, end) = find_sequence(&nums, bad);
    let res = sum_bads(&nums[start..end]);
    println!("res = {}", res);
    Ok(())
}
