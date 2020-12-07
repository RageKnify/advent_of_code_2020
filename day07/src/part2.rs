use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

fn count(map: &HashMap<String, HashSet<(usize, String)>>, target: &str) -> usize {
    let mut sum = 1;
    if let Some(entry) = map.get(target) {
        for (amount, child) in entry {
            sum += amount * count(map, child);
        }
    }
    sum
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut map = HashMap::<String, HashSet<(usize, String)>>::new();
    // Modified from part1 to capture the amounts
    let re: Regex = Regex::new(r#"([A-Za-z ]+) bags contain (\d+) ([A-Za-z ]+) bags?(?:, (\d+) ([A-Za-z ]+) bags?)?(?:, (\d+) ([A-Za-z ]+) bags?)?(?:, (\d+) ([A-Za-z ]+) bags?)?(?:, (\d+) ([A-Za-z ]+) bags?)?"#).unwrap();
    for line in file.lines() {
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let container = caps.get(1).unwrap().as_str();
            let mut set = HashSet::new();

            for i in 1..=5 {
                if let Some(amount) = caps.get(i * 2) {
                    let amount = usize::from_str_radix(amount.as_str(), 10).unwrap();
                    if let Some(contained) = caps.get((i * 2) + 1) {
                        set.insert((amount, contained.as_str().to_owned()));
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            map.insert(container.to_owned(), set);
        }
    }
    // subtract 1 to ignore shiny gold
    println!("res = {}", count(&map, "shiny gold") - 1);
    Ok(())
}
