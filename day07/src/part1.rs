use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

fn count(
    map: &HashMap<String, HashSet<String>>,
    target: &str,
    seen: &mut HashSet<String>,
) -> usize {
    if seen.contains(target) {
        return 0;
    }
    let mut sum = 0;
    seen.insert(target.to_owned());
    for container in map.get(target).unwrap() {
        sum += count(map, container, seen);
    }
    sum + 1
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut map = HashMap::<String, HashSet<String>>::new();
    let re: Regex = Regex::new(r#"([A-Za-z ]+) bags contain \d+ ([A-Za-z ]+) bags?(?:, \d+ ([A-Za-z ]+) bags?)?(?:, \d+ ([A-Za-z ]+) bags?)?(?:, \d+ ([A-Za-z ]+) bags?)?(?:, \d+ ([A-Za-z ]+) bags?)?"#).unwrap();
    for line in file.lines() {
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let container = caps.get(1).unwrap().as_str();
            // Add to map if not there yet
            map.entry(container.to_owned()).or_insert(HashSet::new());

            // Add itself to its children
            for i in 2..=6 {
                if let Some(contained) = caps.get(i) {
                    let set = map
                        .entry(contained.as_str().to_owned())
                        .or_insert(HashSet::new());
                    set.insert(container.to_owned());
                } else {
                    break;
                }
            }
        }
    }
    let mut seen = HashSet::<String>::new();
    // subtract 1 to ignore shiny gold
    println!("res = {}", count(&map, "shiny gold", &mut seen) - 1);
    Ok(())
}
