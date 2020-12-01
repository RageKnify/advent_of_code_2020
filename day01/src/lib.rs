use std::cmp::Ordering;

pub fn part1(nums: &[u32], value: u32) -> Option<(u32, u32)> {
    let mut iter = nums.iter().cloned();
    let mut left = iter.next()?;
    let mut right = iter.next_back()?;
    loop {
        let res = left + right;
        match res.cmp(&value) {
            Ordering::Less => {
                left = iter.next()?;
            }
            Ordering::Greater => {
                right = iter.next_back()?;
            }
            Ordering::Equal => {
                return Some((left, right));
            }
        }
    }
}
