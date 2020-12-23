use std::collections::VecDeque;
use std::fs;
use std::io;

#[derive(Debug)]
struct State {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl State {
    fn finished(&self) -> Option<usize> {
        if self.player1.len() == 0 {
            Some(calculate(&self.player2))
        } else if self.player2.len() == 0 {
            Some(calculate(&self.player1))
        } else {
            None
        }
    }

    fn advance(&mut self) {
        let card1 = self.player1.pop_front().unwrap();
        let card2 = self.player2.pop_front().unwrap();
        match card1 > card2 {
            true => {
                self.player1.push_back(card1);
                self.player1.push_back(card2);
            }
            false => {
                self.player2.push_back(card2);
                self.player2.push_back(card1);
            }
        }
    }
}

fn calculate(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum()
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut decks: Vec<Vec<usize>> = file
        .split("\n\n")
        .map(str::lines)
        .map(|lines| {
            lines
                .skip(1)
                .map(|line| usize::from_str_radix(line, 10))
                .map(Result::unwrap)
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut state = State {
        player2: decks.pop().unwrap().into(),
        player1: decks.pop().unwrap().into(),
    };
    loop {
        if let Some(res) = state.finished() {
            println!("res = {}", res);
            break;
        }
        state.advance()
    }
    Ok(())
}
