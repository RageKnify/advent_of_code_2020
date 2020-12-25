use std::fs;
use std::io;

const N: u64 = 20201227;
const G: u64 = 7;

fn discrete_log(door_pub: u64) -> u64 {
    let mut c = G;
    let mut i = 1;
    while c != door_pub {
        c = (c * G) % N;
        i += 1;
    }
    i
}

fn pow(b: u64, e: u64) -> u64 {
    let mut c = b;
    for _ in 1..e {
        c = (c * b) % N;
    }
    c
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut lines = file.lines();
    let card_pub = u64::from_str_radix(lines.next().unwrap(), 10).unwrap();
    let door_pub = u64::from_str_radix(lines.next().unwrap(), 10).unwrap();
    let door_priv = discrete_log(door_pub);
    let res = pow(card_pub, door_priv);
    println!("res = {}", res);
    Ok(())
}
