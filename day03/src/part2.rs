use std::fs;
use std::io;

struct Forest {
    // A lot of allocations, but it works
    lines: Vec<Vec<u8>>,
}

impl Forest {
    fn tree(&self, x: usize, y: usize) -> bool {
        let width = self.lines[0].len();
        self.lines[y][x % width] == b'#'
    }
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    // let file = fs::read_to_string("test.txt")?;
    let forest = Forest {
        lines: file.lines().map(|l| l.to_owned().into_bytes()).collect(),
    };
    let n = forest.lines.len();
    let mut res = (1..n).filter(|y| forest.tree(*y, *y)).count();
    res *= (1..n).filter(|y| forest.tree(3 * y, *y)).count();
    res *= (1..n).filter(|y| forest.tree(5 * y, *y)).count();
    res *= (1..n).filter(|y| forest.tree(7 * y, *y)).count();
    // I this one could have failed, but it works...
    res *= (1..n / 2).filter(|y| forest.tree(*y, 2 * y)).count();
    println!("res = {}", res);
    Ok(())
}
