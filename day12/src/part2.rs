use std::convert::TryInto;
use std::fs;
use std::io;

fn eval(file: &str) -> usize {
    let mut way_x: i32 = 10;
    let mut way_y: i32 = 1;
    let mut ship_y: i32 = 0;
    let mut ship_x: i32 = 0;
    for l in file.lines().map(str::as_bytes) {
        let amount = i32::from_str_radix(std::str::from_utf8(&l[1..]).unwrap(), 10).unwrap();
        match l[0] {
            b'N' => way_y += amount,
            b'S' => way_y -= amount,
            b'E' => way_x += amount,
            b'W' => way_x -= amount,
            b'L' => {
                let count = amount / 90;
                for _ in 0..count {
                    let old_way_x = way_x;
                    way_x = -1 * way_y;
                    way_y = old_way_x;
                }
            }
            b'R' => {
                let count = amount / 90;
                for _ in 0..count {
                    let old_way_x = way_x;
                    way_x = way_y;
                    way_y = -1 * old_way_x;
                }
            }
            b'F' => {
                ship_x += way_x * amount;
                ship_y += way_y * amount;
            }
            _ => unreachable!(),
        };
    }
    (ship_y.abs() + ship_x.abs()).try_into().unwrap()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    println!("res = {}", eval(&file));
    Ok(())
}
