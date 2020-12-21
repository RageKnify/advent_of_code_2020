use std::fs;
use std::io;

fn open_parens(line: &[u8], closing: usize) -> usize {
    let mut inner = 0;
    for (idx, c) in line[..closing].iter().rev().enumerate() {
        if *c == b')' {
            inner += 1;
        }
        if *c == b'(' {
            if inner > 0 {
                inner -= 1;
            } else {
                let open = closing - (idx + 1);
                return open;
            }
        }
    }
    unreachable!()
}

fn eval_recursive(line: &[u8], idx: &mut usize) -> u64 {
    let right = if line[*idx] == b')' {
        let parens_idx = open_parens(line, *idx);

        let inner = &line[parens_idx + 1..*idx];
        *idx = parens_idx;

        let mut inner_idx = inner.len() - 1;
        eval_recursive(inner, &mut inner_idx)
    } else {
        let end = *idx;

        while *idx >= 1 && line[*idx - 1] <= b'9' && line[*idx - 1] >= b'0' {
            *idx -= 1;
        }

        let range = *idx..=end;

        u64::from_str_radix(std::str::from_utf8(&line[range]).unwrap(), 10).unwrap()
    };

    if *idx == 0 {
        return right;
    }

    // space + last
    *idx -= 2;

    // operation
    let op = line[*idx];

    // skip space
    *idx -= 1;
    // left
    *idx -= 1;
    let left = eval_recursive(line, idx);

    match op {
        b'+' => left + right,
        b'*' => left * right,
        _ => unreachable!(),
    }
}

fn eval_line(line: &str) -> u64 {
    let mut idx = line.len() - 1;
    eval_recursive(line.as_bytes(), &mut idx)
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let res: u64 = file.lines().map(eval_line).sum();
    println!("res = {}", res);
    Ok(())
}
