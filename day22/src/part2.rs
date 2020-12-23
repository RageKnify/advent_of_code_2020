use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

fn calculate(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum()
}

impl State {
    fn finished(&self) -> Option<(bool, usize)> {
        if self.player1.len() == 0 {
            Some((true, calculate(&self.player2)))
        } else if self.player2.len() == 0 {
            Some((false, calculate(&self.player1)))
        } else {
            None
        }
    }

    fn advance(&mut self) {
        let card1 = self.player1.pop_front().unwrap();
        let card2 = self.player2.pop_front().unwrap();
        let player1victory;
        if self.player1.len() >= card1 && self.player2.len() >= card2 {
            let player1 = self.player1.iter().take(card1).cloned().collect();
            let player2 = self.player2.iter().take(card2).cloned().collect();
            let inner = State { player1, player2 };
            let big = BigState {
                inner,
                old_states: HashSet::new(),
            };
            let (player2victory, _) = big.result();
            player1victory = !player2victory;
        } else {
            player1victory = card1 > card2;
        }
        if player1victory {
            self.player1.push_back(card1);
            self.player1.push_back(card2);
        } else {
            self.player2.push_back(card2);
            self.player2.push_back(card1);
        }
    }
}

#[derive(Debug)]
struct BigState {
    inner: State,
    old_states: HashSet<State>,
}

impl BigState {
    fn finished(&self) -> Option<(bool, usize)> {
        if self.old_states.contains(&self.inner) {
            return Some((false, calculate(&self.inner.player1)));
        }
        self.inner.finished()
    }

    fn advance(&mut self) {
        self.old_states.insert(self.inner.clone());
        self.inner.advance();
    }

    fn result(mut self) -> (bool, usize) {
        loop {
            if let Some(tup) = self.finished() {
                return tup;
            }
            self.advance();
        }
    }
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
    let state = State {
        player2: decks.pop().unwrap().into(),
        player1: decks.pop().unwrap().into(),
    };
    let big = BigState {
        inner: state,
        old_states: HashSet::new(),
    };
    let (_, res) = big.result();
    println!("res = {}", res);
    Ok(())
}
