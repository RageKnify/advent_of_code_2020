use std::collections::HashSet;
use std::fs;
use std::io;

fn part1(group: &str) -> usize {
    let found: HashSet<_> = group
        .as_bytes()
        .iter()
        .filter(|x| b'a' <= **x && **x <= b'z')
        .collect();
    found.len()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let groups = file.split("\n\n");
    let res: usize = groups.map(part1).sum();
    println!("res = {}", res);
    Ok(())
}
