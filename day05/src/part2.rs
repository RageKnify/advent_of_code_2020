use std::convert::TryInto;
use std::fs;
use std::io;

fn calculate(line: [u8; 10]) -> usize {
    let row = {
        let (mut min, mut max) = (0, 127);
        for b in &line[..7] {
            let mid = max - (max - min) / 2;
            match b {
                b'F' => max = mid,
                b'B' => min = mid,
                _ => unreachable!(),
            }
        }
        min
    };
    let col = {
        let (mut min, mut max) = (0, 7);
        for b in &line[7..10] {
            let mid = max - (max - min) / 2;
            match b {
                b'L' => max = mid,
                b'R' => min = mid,
                _ => unreachable!(),
            }
        }
        min
    };
    row * 8 + col
}

fn main() -> io::Result<()> {
    let mut res: Vec<usize> = fs::read_to_string("input.txt")?
        .trim()
        .lines()
        .map(|line| calculate(line.as_bytes().try_into().unwrap()))
        .collect();
    res.sort();
    let mut last = res.pop().unwrap();
    while let Some(curr) = res.pop() {
        if last - 1 > curr {
            println!("res = {:?}", last - 1);
            break;
        }
        last = curr;
    }
    Ok(())
}
