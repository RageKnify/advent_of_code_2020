use std::fs;
use std::io;

fn valid<'string>(passport: &'string &str) -> bool {
    let starts: Vec<_> = passport
        .split(|c| c == ' ' || c == '\n')
        .map(|x| x.split(':').next().unwrap())
        .collect();
    let byr = starts.contains(&"byr");
    let iyr = starts.contains(&"iyr");
    let eyr = starts.contains(&"eyr");
    let hgt = starts.contains(&"hgt");
    let hcl = starts.contains(&"hcl");
    let ecl = starts.contains(&"ecl");
    let pid = starts.contains(&"pid");
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let passports = file.split("\n\n");
    let res = passports.filter(valid).count();
    println!("res = {}", res);
    Ok(())
}
