use std::convert::TryInto;
use std::fs;
use std::io;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Add<i32> for Direction {
    type Output = Self;

    fn add(mut self, mut changes: i32) -> Self {
        assert!(changes < 4, "angle needs to be multiple of 90");
        while changes > 0 {
            changes -= 1;
            self = match self {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            }
        }
        self
    }
}

impl Sub<i32> for Direction {
    type Output = Self;

    fn sub(mut self, mut changes: i32) -> Self {
        assert!(changes < 4, "angle needs to be multiple of 90");
        while changes > 0 {
            changes -= 1;
            self = match self {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
            }
        }
        self
    }
}

fn eval(file: &str) -> usize {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut dir = Direction::East;
    for l in file.lines().map(str::as_bytes) {
        let amount = i32::from_str_radix(std::str::from_utf8(&l[1..]).unwrap(), 10).unwrap();
        match l[0] {
            b'N' => y += amount,
            b'S' => y -= amount,
            b'E' => x += amount,
            b'W' => x -= amount,
            b'L' => dir = dir - (amount / 90),
            b'R' => dir = dir + (amount / 90),
            b'F' => match dir {
                Direction::North => y += amount,
                Direction::South => y -= amount,
                Direction::East => x += amount,
                Direction::West => x -= amount,
            },
            _ => unreachable!(),
        };
    }
    (y.abs() + x.abs()).try_into().unwrap()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    println!("res = {}", eval(&file));
    Ok(())
}
