use std::collections::HashSet;
use std::fs;
use std::io;

fn part2(group: &str) -> usize {
    let mut lines = group.lines();
    let mut questions: HashSet<_> = lines.next().unwrap().as_bytes().iter().collect();
    for line in lines {
        questions.retain(|q| line.as_bytes().contains(q));
    }
    questions.len()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let groups = file.split("\n\n");
    let res: usize = groups.map(part2).sum();
    println!("res = {}", res);
    Ok(())
}
