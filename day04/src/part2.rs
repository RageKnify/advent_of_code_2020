use std::io;
use std::{collections::HashMap, fs};

const COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn valid<'string>(passport: &'string &str) -> bool {
    let pairs: HashMap<_, _> = passport
        .trim()
        .split(|c| c == ' ' || c == '\n')
        .map(|x| {
            let mut tmp = x.split(':');
            (tmp.next().unwrap(), tmp.next().unwrap())
        })
        .collect();
    let byr = pairs
        .get(&"byr")
        .map(|n| n.len() == 4 && (1920..=2002).contains(&usize::from_str_radix(n, 10).unwrap_or(0)))
        .unwrap_or(false);
    let iyr = pairs
        .get(&"iyr")
        .map(|n| n.len() == 4 && (2010..=2020).contains(&usize::from_str_radix(n, 10).unwrap_or(0)))
        .unwrap_or(false);
    let eyr = pairs
        .get(&"eyr")
        .map(|n| n.len() == 4 && (2020..=2030).contains(&usize::from_str_radix(n, 10).unwrap_or(0)))
        .unwrap_or(false);
    let hgt = pairs
        .get(&"hgt")
        .map(|string| {
            if string.ends_with("in") {
                usize::from_str_radix(string.strip_suffix("in").unwrap(), 10)
                    .map(|n| (59..=76).contains(&n))
                    .unwrap_or(false)
            } else if string.ends_with("cm") {
                usize::from_str_radix(string.strip_suffix("cm").unwrap(), 10)
                    .map(|n| (150..=193).contains(&n))
                    .unwrap_or(false)
            } else {
                false
            }
        })
        .unwrap_or(false);
    let hcl = pairs
        .get(&"hcl")
        .map(|h_str| {
            h_str.as_bytes()[0] == b'#'
                && usize::from_str_radix(std::str::from_utf8(&h_str.as_bytes()[1..]).unwrap(), 16)
                    .is_ok()
        })
        .unwrap_or(false);
    let ecl = pairs
        .get(&"ecl")
        .map(|x| COLORS.contains(x))
        .unwrap_or(false);
    let pid = pairs
        .get(&"pid")
        .map(|n_str| n_str.len() == 9 && usize::from_str_radix(n_str, 10).is_ok())
        .unwrap_or(false);
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let passports = file.split("\n\n");
    let res = passports.filter(valid).count();
    println!("res = {}", res);
    Ok(())
}
