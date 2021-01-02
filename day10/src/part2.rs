use std::collections::HashMap;
use std::fs;
use std::io;

fn part2(joltages: &[usize]) -> usize {
    let mut known = HashMap::new();
    for i in (0..joltages.len()).rev() {
        let curr = joltages[i];
        known.insert(
            curr,
            if i == joltages.len() - 1 {
                1
            } else {
                let mut res = 0;
                for j in (i + 1)..(std::cmp::min(i + 4, joltages.len())) {
                    let next = joltages[j];
                    if next - curr <= 3 {
                        res += known[&next];
                    }
                }
                res
            },
        );
    }
    known[&0]
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    // charging outlet
    let mut joltages = vec![0];

    let lines = file.lines();
    joltages.reserve_exact(lines.size_hint().0);
    joltages.extend(lines.map(|line| usize::from_str_radix(line, 10).unwrap()));

    joltages.sort();
    // buil-in adapter
    let last = *joltages.last().unwrap();
    joltages.push(last + 3);

    let res = part2(&joltages);
    println!("res = {}", res);
    Ok(())
}
