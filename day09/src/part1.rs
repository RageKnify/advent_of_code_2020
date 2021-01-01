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

fn find_bad(nums: Vec<usize>) -> usize {
    let mut i = 0;
    loop {
        let curr = nums[i + 25];
        if is_bad((nums[i..i + 25]).try_into().unwrap(), curr) {
            return curr;
        }
        i += 1;
    }
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let nums: Vec<_> = file
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();
    let bad = find_bad(nums);
    println!("res = {}", bad);
    Ok(())
}
