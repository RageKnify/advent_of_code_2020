use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

static NEIGHBOURS: [(i8, i8, i8, i8); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate(i8, i8, i8, i8);

impl Coordinate {
    fn neighbours(&self) -> Vec<Coordinate> {
        let mut res = Vec::new();
        let (x, y, z, w) = (self.0, self.1, self.2, self.3);
        for (dx, dy, dz, dw) in &NEIGHBOURS {
            res.push(Coordinate(x + dx, y + dy, z + dz, w + dw));
        }
        res
    }
}

fn count_neighbors(current: &HashSet<Coordinate>) -> HashMap<Coordinate, usize> {
    let mut counts = HashMap::new();
    for active in current {
        for neighbour in active.neighbours() {
            *counts.entry(neighbour).or_insert(0) += 1;
        }
    }
    counts
}

fn next(current: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    count_neighbors(current)
        .iter()
        .filter(
            |(pos, neighbours)| match (current.contains(pos), neighbours) {
                (true, 2) | (_, 3) => true,
                _ => false,
            },
        )
        .map(|(&pos, _)| pos)
        .collect()
}

fn main() -> io::Result<()> {
    let mut active = HashSet::new();
    let file = fs::read_to_string("input.txt")?;
    for (y, line) in file.lines().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            let coordinate = Coordinate(x as i8, y as i8, 0, 0);
            active.insert(coordinate);
        }
    }
    for _ in 0..6 {
        active = next(&active);
    }
    println!("res = {}", active.len());
    Ok(())
}
