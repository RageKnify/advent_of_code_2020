use ring_algorithm::chinese_remainder_theorem;
use std::fs;
use std::io;

fn part2(bus_ids: &[isize]) -> isize {
    let mut v = Vec::new();
    for (i, val) in bus_ids.iter().enumerate() {
        v.push(if *val == 0 {
            1
        } else {
            // val - i ≡val
            ((val - i as isize) % val + val) % val
        });
    }
    chinese_remainder_theorem(&v, &bus_ids).unwrap()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut lines = file.lines();
    let _ = lines.next();
    let bus_ids: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| isize::from_str_radix(s, 10).unwrap_or(1))
        .collect();
    let res = part2(&bus_ids);
    println!("res = {}", res);
    Ok(())
}
