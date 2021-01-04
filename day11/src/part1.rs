use std::fmt;
use std::fs;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spot {
    Empty,
    Floor,
    Occupied,
}

impl fmt::Debug for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Spot::Floor => ".",
            Spot::Empty => "L",
            Spot::Occupied => "#",
        })
    }
}

impl From<&u8> for Spot {
    fn from(byte: &u8) -> Self {
        match byte {
            b'L' => Spot::Empty,
            b'.' => Spot::Floor,
            b'#' => Spot::Occupied,
            _ => unreachable!(),
        }
    }
}

impl From<&Spot> for char {
    fn from(s: &Spot) -> char {
        match s {
            Spot::Floor => '.',
            Spot::Empty => 'L',
            Spot::Occupied => '#',
        }
    }
}

struct Room {
    seats: Vec<Vec<Spot>>,
}

impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for line in &self.seats {
            out.extend(line.iter().map(char::from));
            out.push('\n');
        }
        f.write_str(&out)
    }
}

impl Index<usize> for Room {
    type Output = Vec<Spot>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.seats[idx]
    }
}

impl IndexMut<usize> for Room {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.seats[idx]
    }
}

impl Room {
    fn calc_neighbours(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        for (xi, yi) in DELTAS
            .iter()
            .map(|(dx, dy)| (dx + x as isize, dy + y as isize))
        {
            if xi >= 0 && yi >= 0 {
                match self.seats.get(yi as usize) {
                    Some(line) => match line.get(xi as usize) {
                        Some(spot) => match spot {
                            Spot::Occupied => sum += 1,
                            Spot::Empty => {}
                            Spot::Floor => {}
                        },
                        None => {}
                    },
                    None => {}
                }
            }
        }
        sum
    }
}

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn next(room: &Room) -> (Room, bool) {
    let mut same = true;
    let mut next: Vec<Vec<Spot>> = Vec::new();
    next.extend(room.seats.iter().map(Clone::clone));
    let mut next = Room { seats: next };
    for (y, line) in room.seats.iter().enumerate() {
        for (x, spot) in line.iter().enumerate() {
            let active_neighbours = room.calc_neighbours(x, y);
            match (spot, active_neighbours) {
                (Spot::Empty, 0) => {
                    same = false;
                    next[y][x] = Spot::Occupied;
                }
                (Spot::Occupied, active_neighbours) if active_neighbours >= 4 => {
                    same = false;
                    next[y][x] = Spot::Empty;
                }
                _ => (),
            }
        }
    }
    (next, same)
}

fn part1(room: Room) -> usize {
    let (mut room, mut finished) = (room, false);
    while !finished {
        let x = next(&room);
        room = x.0;
        finished = x.1;
    }
    room.seats
        .iter()
        .map(|x| x.iter())
        .flatten()
        .filter(|s| **s == Spot::Occupied)
        .count()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let room = Room {
        seats: file
            .lines()
            .map(str::as_bytes)
            .map(|x| x.iter().map(Spot::from).collect())
            .collect(),
    };
    let res = part1(room);
    println!("res = {}", res);
    Ok(())
}
