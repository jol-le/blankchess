use std::fmt::Error;

use crate::pieces::*;

pub const SQUARE_SIZE: f32 = 100.0;

pub const STARTING_POSITION: &'static str =
    "rnbqkbnr/pppppppp/xxxxxxxx/xxxxxxxx/xxxxxxxx/xxxxxxxx/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub type Fen = String;

#[derive(Debug, Copy, Clone)]
pub struct Square(pub u8, pub u8);

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = match self.0 {
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e",
            6 => "f",
            7 => "g",
            8 => "h",
            _ => "",
        };
        let col = match self.1 {
            1 => "8",
            2 => "7",
            3 => "6",
            4 => "5",
            5 => "4",
            6 => "3",
            7 => "2",
            8 => "1",
            _ => "",
        };

        write!(f, "{}{}", row, col)
    }
}

pub struct Moves {
    pub state: Fen,
    movelist: Vec<Move>,
}

impl Default for Moves {
    fn default() -> Self {
        Moves {
            state: STARTING_POSITION.to_string(),
            movelist: vec![],
        }
    }
}

impl Moves {
    pub fn get_pgn(&self) -> String {
        self.movelist
            .iter()
            .enumerate()
            .map(|(count, item)| {
                let mut s = item.to_string();
                if count % 2 == 0 {
                    s = (count / 2 + 1).to_string() + ". " + &s;
                }
                s
            })
            .collect::<String>()
    }

    // Errortypes InvalidMove etc.
    pub fn make_move(&mut self, change: Move) -> Result<(), Error> {
        if !self.check_valid_move(change) {
            // [TODO] Richtiges Handling
            return Err(Error);
        }

        self.movelist.push(change);
        self.state = self.state_update(change);
        Ok(())
    }

    fn check_valid_move(&self, change: Move) -> bool {
        let valid_move = match change.piece.kind {
            PieceKind::King => change.is_kingmove(self.state.clone()),
            PieceKind::Queen => change.is_queenmove(self.state.clone()),
            PieceKind::Bishop => change.is_bishopmove(self.state.clone()),
            PieceKind::Knight => change.is_knightmove(self.state.clone()),
            PieceKind::Rook => change.is_rookmove(self.state.clone()),
            PieceKind::Pawn => change.is_pawnmove(self.state.clone(), self.movelist.last()),
            _ => true,
        };

        valid_move
    }

    fn state_update(&self, change: Move) -> String {
        let mut state = String::new();

        let from_square = ((change.from.1 - 1) * 9) + (change.from.0 - 1);
        let to_square = ((change.to.1 - 1) * 9) + (change.to.0 - 1);

        for (i, c) in self.state.clone().chars().enumerate() {
            if i == from_square as usize {
                state.push('x');
            } else if i == to_square as usize {
                state.push(change.piece.into());
            } else {
                state.push(c);
            }
        }

        state
    }
}
