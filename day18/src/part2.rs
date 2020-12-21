use std::fs;
use std::io;

use crate::arithmetic::expression;
use peg;

peg::parser!( grammar arithmetic() for str {
        pub rule expression() -> i64
        = product()

    rule sum() -> i64
        = l:value() " + " r:sum() { l+r }
        / value()

    rule product() -> i64
        = l:sum() " * " r:product() { l*r }
        / sum()

    rule value() -> i64
        = number()
        / "(" v:expression() ")" { v }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }
});

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let res: i64 = file.lines().map(expression).map(Result::unwrap).sum();
    println!("res = {}", res);
    Ok(())
}
