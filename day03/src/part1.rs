use std::fs;
use std::io;

struct Forest {
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
    let res = (1..n).filter(|y| forest.tree(3 * y, *y)).count();
    println!("res = {}", res);
    Ok(())
}
