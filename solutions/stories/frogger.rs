// We gaan in deze oefening het spel Frogger spelen
// Hier kun je lezen hoe het werkt: https://open.kattis.com/problems/1dfroggereasy
// Het inlezen van het bord is al gedaan en merk op dat we zoveel mogelijk
// het type systeem van Rust gebruiken om het spel te modelleren.
// Aan jou de taak om de logiva van het spel te implementeren, zodat de uitkomst klopt.

use std::{collections::HashSet, convert::TryFrom, fmt::Display};

use anyhow::bail;

fn main() {}

struct Frogger {
    board: Vec<i32>,
    starting_index: usize,
    magic_number: i32,
}

impl Frogger {
    fn new(board: Vec<i32>, starting_index: usize, magic_number: i32) -> anyhow::Result<Self> {
        // We controleren alle precondities, zodat we zeker weten met een correcte toestand te werken
        if board.is_empty() {
            bail!("You cannot play on an empty board")
        }
        if starting_index >= board.len() {
            bail!("You need to start in a valid location on the board")
        }
        if board.contains(&0) {
            bail!("The board may not contain the value 0 on any tile")
        }
        if board.iter().all(|value| *value != magic_number) {
            bail!("There is no position containing the magic number")
        }
        Ok(Frogger {
            board,
            starting_index,
            magic_number,
        })
    }

    fn play(&self) -> (Fate, usize) {
        // TODO: Gebruik FroggerBoard om de eindtoestand (Fate) te bepalen
        let mut frogger_board =
            FroggerBoard::new(self.starting_index, &self.board, self.magic_number);
        // We itereren door herhaaldelijk een stap te doen op het bord totdat we een eindtoestand bereiken
        let (hops, fate) = std::iter::from_fn(|| Some(frogger_board.step()))
            .enumerate()
            .find(|(_, maybe_fate)| maybe_fate.is_some())
            .unwrap();
        (fate.unwrap(), hops)
    }
}

struct FroggerBoard<'a> {
    position: BoardPosition<'a>,
    visited: HashSet<usize>,
    magic_number: i32,
}

impl<'a> FroggerBoard<'a> {
    fn new(starting_index: usize, board: &'a Vec<i32>, magic_number: i32) -> Self {
        // We houden een referentie naar het bord vast, zolang we ons op het bord bevinden
        Self {
            position: BoardPosition::new(starting_index, board),
            visited: HashSet::new(),
            magic_number,
        }
    }

    fn step(&mut self) -> Option<Fate> {
        let maybe_fate = match self.position {
            BoardPosition::Valid(index, _) => {
                if !self.visited.insert(index) {
                    Some(Fate::Cycle)
                } else if self.position.get_value() == Some(self.magic_number) {
                    Some(Fate::Magic)
                } else {
                    None
                }
            }
            BoardPosition::Left => Some(Fate::Left),
            BoardPosition::Right => Some(Fate::Right),
        };
        if maybe_fate.is_none() {
            // We zetten alleen een stap als we ons niet in een eindtoestand bevinden
            self.position.step();
        }
        maybe_fate
    }
}

// Er zijn drie mogelijke locaties waar we ons ten opzichte van het bord kunnen bevinden
// Alleen wanneer we op het bord staan bevinden we ons in een geldige positie
enum BoardPosition<'a> {
    Valid(usize, &'a Vec<i32>),
    Left,
    Right,
}

impl<'a> BoardPosition<'a> {
    fn new(starting_index: usize, board: &'a Vec<i32>) -> Self {
        if starting_index <= board.len() {
            Self::Valid(starting_index, board)
        } else {
            // Dit komt in de praktijk niet voor, omdat we een start positie buiten
            // het bord al afvlaggen als fout bij het aanmaken van Frogger.
            // Vanuit het perspectief va het bord is het echter niet fout: we
            // bevinden ons immers rechts van het bord.
            Self::Right
        }
    }

    fn step(&mut self) {
        // We hoeven de interne toestand alleen te updaten als we ons op het bord bevinden
        if let Self::Valid(index, board) = *self {
            let new_index = i32::try_from(index).unwrap() + board[index];
            if new_index < 0 {
                *self = Self::Left;
                return;
            }
            let new_index = usize::try_from(new_index).unwrap();
            if new_index >= board.len() {
                *self = Self::Right;
            } else {
                *self = Self::Valid(new_index, board);
            }
        }
    }

    fn get_value(&self) -> Option<i32> {
        match self {
            BoardPosition::Valid(index, board) => Some(board[*index]),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Fate {
    Magic,
    Left,
    Right,
    Cycle,
}

impl Display for Fate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Fate::Magic => "magic",
            Fate::Left => "left",
            Fate::Right => "right",
            Fate::Cycle => "cycle",
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_1() {
        let frogger = Frogger::new(vec![-9, 1, 42, -2, -3, -3], 4, 42).unwrap();
        assert_eq!(frogger.play(), (Fate::Magic, 2));
    }

    #[test]
    fn sample_2() {
        let frogger = Frogger::new(vec![7, 5, 4, 2, 13, -2, -3, 6], 2, 13).unwrap();
        assert_eq!(frogger.play(), (Fate::Cycle, 4));
    }

    #[test]
    fn fall_off_left() {
        let frogger = Frogger::new(vec![-1, 2], 0, 2).unwrap();
        assert_eq!(frogger.play(), (Fate::Left, 1));
    }

    #[test]
    fn fall_off_right() {
        let frogger = Frogger::new(vec![2, 1], 0, 1).unwrap();
        assert_eq!(frogger.play(), (Fate::Right, 1));
    }

    #[test]
    fn start_on_magic_number() {
        let frogger = Frogger::new(vec![1, 2, 3], 1, 2).unwrap();
        assert_eq!(frogger.play(), (Fate::Magic, 0));
    }

    #[test]
    fn empty_board() {
        assert!(Frogger::new(vec![], 0, 2).is_err());
    }

    #[test]
    fn starting_position_not_on_board() {
        assert!(Frogger::new(vec![5], 1, 5).is_err());
    }

    #[test]
    fn magic_number_not_on_board() {
        assert!(Frogger::new(vec![4], 0, 5).is_err());
    }

    #[test]
    fn board_cannot_contain_zero() {
        assert!(Frogger::new(vec![1, 2, 0, 4, 5], 0, 5).is_err());
    }
}
