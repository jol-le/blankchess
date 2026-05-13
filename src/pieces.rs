use eframe::egui::{self, Response, Sense, Ui};

use crate::board::*;

#[derive(Debug, Copy, Clone)]
pub(crate) enum PieceKind {
    King,
    Queen,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum PieceColor {
    Black,
    White,
    None,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Piece {
    pub(crate) kind: PieceKind,
    pub(crate) color: PieceColor,
}

impl Piece {
    pub(crate) fn new(c: char) -> Self {
        match c {
            'K' => Piece {
                kind: PieceKind::King,
                color: PieceColor::White,
            },
            'k' => Piece {
                kind: PieceKind::King,
                color: PieceColor::Black,
            },
            'Q' => Piece {
                kind: PieceKind::Queen,
                color: PieceColor::White,
            },
            'q' => Piece {
                kind: PieceKind::Queen,
                color: PieceColor::Black,
            },
            'B' => Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::White,
            },
            'b' => Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::Black,
            },
            'N' => Piece {
                kind: PieceKind::Knight,
                color: PieceColor::White,
            },
            'n' => Piece {
                kind: PieceKind::Knight,
                color: PieceColor::Black,
            },
            'R' => Piece {
                kind: PieceKind::Rook,
                color: PieceColor::White,
            },
            'r' => Piece {
                kind: PieceKind::Rook,
                color: PieceColor::Black,
            },
            'P' => Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            },
            'p' => Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::Black,
            },
            _ => Piece {
                kind: PieceKind::Empty,
                color: PieceColor::None,
            },
        }
    }

    pub(crate) fn draw_piece(&self, ui: &mut Ui) -> Response {
        match self.color {
            PieceColor::Black => match self.kind {
                PieceKind::King => ui.add(
                    egui::Image::new(egui::include_image!("../assets/black/king.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE))
                        .sense(Sense::click()),
                ),
                PieceKind::Queen => ui.add(
                    egui::Image::new(egui::include_image!("../assets/black/queen.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Bishop => ui.add(
                    egui::Image::new(egui::include_image!("../assets/black/bishop.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Knight => ui.add(
                    egui::Image::new(egui::include_image!("../assets/black/knight.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Rook => ui.add(
                    egui::Image::new(egui::include_image!("../assets/black/rook.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Pawn => ui.add(
                    egui::Image::new(egui::include_image!("../assets/black/pawn.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                _ => {
                    unreachable!("Empty does not have a color!")
                }
            },
            PieceColor::White => match self.kind {
                PieceKind::King => ui.add(
                    egui::Image::new(egui::include_image!("../assets/white/king.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Queen => ui.add(
                    egui::Image::new(egui::include_image!("../assets/white/queen.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Bishop => ui.add(
                    egui::Image::new(egui::include_image!("../assets/white/bishop.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Knight => ui.add(
                    egui::Image::new(egui::include_image!("../assets/white/knight.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Rook => ui.add(
                    egui::Image::new(egui::include_image!("../assets/white/rook.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                PieceKind::Pawn => ui.add(
                    egui::Image::new(egui::include_image!("../assets/white/pawn.png"))
                        .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ),
                _ => {
                    unreachable!("Empty does not have a color!")
                }
            },
            PieceColor::None => ui.add(
                egui::Image::new(egui::include_image!("../assets/empty.png"))
                    .fit_to_exact_size(egui::Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
            ),
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        }
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value.color {
            PieceColor::Black => match value.kind {
                PieceKind::King => 'k',
                PieceKind::Queen => 'q',
                PieceKind::Bishop => 'b',
                PieceKind::Knight => 'n',
                PieceKind::Rook => 'r',
                PieceKind::Pawn => 'p',
                PieceKind::Empty => 'x',
            },
            PieceColor::White => match value.kind {
                PieceKind::King => 'K',
                PieceKind::Queen => 'Q',
                PieceKind::Bishop => 'B',
                PieceKind::Knight => 'N',
                PieceKind::Rook => 'R',
                PieceKind::Pawn => 'P',
                PieceKind::Empty => 'x',
            },
            _ => 'x',
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self.color {
            _ => match self.kind {
                PieceKind::King => "K",
                PieceKind::Queen => "Q",
                PieceKind::Bishop => "B",
                PieceKind::Knight => "N",
                PieceKind::Rook => "R",
                PieceKind::Pawn => "",
                PieceKind::Empty => "",
            },
        };
        write!(f, "{}", piece)
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) piece: Piece,
}

// [TODO] Implementieren von allen Figuren / Zugregeln
impl Move {
    pub(crate) fn is_kingmove(&self, _board: String) -> bool {
        true
    }

    pub(crate) fn is_queenmove(&self, _board: String) -> bool {
        true
    }

    pub(crate) fn is_bishopmove(&self, _board: String) -> bool {
        true
    }

    pub(crate) fn is_knightmove(&self, _board: String) -> bool {
        true
    }

    pub(crate) fn is_rookmove(&self, _board: String) -> bool {
        true
    }

    pub(crate) fn is_pawnmove(&self, _board: String, _prev_move: Option<&Move>) -> bool {
        true
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.piece.color {
            PieceColor::White => write!(f, "{}{} ", self.piece, self.to),
            PieceColor::Black => write!(f, " {}{} ", self.piece, self.to),
            _ => unreachable!("Pieces always have a color!"),
        }
    }
}
