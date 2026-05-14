use eframe::egui::{self, Color32, Event, Id, Pos2, Rect, Vec2};

use crate::board::{Moves, SQUARE_SIZE, STARTING_POSITION, Square};
use crate::pieces::{Move, Piece, PieceColor, PieceKind};

mod board;
mod pieces;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "blankchess",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<BlankChess>::default())
        }),
    )
}

struct BlankChess {
    // Settings
    board_rect: Rect,
    white_color: Color32,
    black_color: Color32,

    // Gameplay
    moves: Moves,
    to_move: PieceColor,
    next_move_from: Option<Square>,
}

impl Default for BlankChess {
    fn default() -> Self {
        BlankChess {
            board_rect: Rect {
                min: Pos2::ZERO,
                max: Pos2::ZERO,
            },
            white_color: Color32::from_hex("#E0CBA8").unwrap(),
            black_color: Color32::from_hex("#60463B").unwrap(),
            moves: Moves::default(),
            to_move: PieceColor::White,
            next_move_from: None,
        }
    }
}

impl BlankChess {
    fn draw_piece(&mut self, ui: &mut egui::Ui, square: Square) {
        let piece: Piece = self.get_piece_at_pos(square);
        piece.draw_piece(ui);
    }

    // [TODO] Soll funktionieren ohne Loops, weil Fen geändert und jetzt einfacher chars zu ersetzen.
    fn get_piece_at_pos(&self, square: Square) -> Piece {
        let mut piece = Piece::new('x');

        let mut row_count = 1;
        let mut col_count = 1;

        for c in self.moves.state.chars() {
            if square.1 == row_count && square.0 as u8 == col_count {
                piece = Piece::new(c);
            }
            if c == '/' {
                row_count += 1;
                col_count = 0;
            }
            col_count += 1;
        }

        piece
    }

    fn get_clicked_square(&self, event: &Event) -> Option<Square> {
        if let Event::PointerButton { pos, .. } = event {
            if !(self.board_rect.min.x <= pos.x
                && pos.x <= self.board_rect.max.x
                && self.board_rect.min.y <= pos.y
                && pos.y <= self.board_rect.max.y)
            {
                return None;
            }

            let mut top = self.board_rect.min.y;
            let mut left = self.board_rect.min.x;

            let mut col = 1u8;
            while top + SQUARE_SIZE <= pos.y && col < 8 {
                top += SQUARE_SIZE;
                col += 1;
            }

            let mut row = 1u8;
            while left + SQUARE_SIZE <= pos.x && row < 8 {
                left += SQUARE_SIZE;
                row += 1;
            }

            return Some(Square(row, col));
        }

        unreachable!("There should have been a mouseclick!");
    }
}

impl eframe::App for BlankChess {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.input(|i| {
            for event in &i.raw.events {
                if matches!(
                    event,
                    Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        ..
                    }
                ) {
                    if self.next_move_from.is_none() {
                        if let Some(next_move) = self.get_clicked_square(event) {
                            if !matches!(
                                self.get_piece_at_pos(next_move),
                                Piece {
                                    kind: PieceKind::Empty,
                                    ..
                                }
                            ) {
                                self.next_move_from = Some(next_move);
                            }
                        }
                    } else {
                        if let Some(to) = self.get_clicked_square(event) {
                            let piece = self.get_piece_at_pos(self.next_move_from.unwrap());

                            if piece.color != self.to_move {
                                println!("Not your Move!"); // [TODO] Richtiges Handling
                                self.next_move_from = None;
                                continue;
                            }

                            match self.moves.make_move(Move {
                                from: self.next_move_from.unwrap(),
                                to,
                                piece,
                            }) {
                                Ok(_) => {
                                    self.to_move = match &self.to_move {
                                        PieceColor::Black => PieceColor::White,
                                        PieceColor::White => PieceColor::Black,
                                        _ => PieceColor::None,
                                    };
                                }
                                Err(_) => println!("Illegal Move"), // [TODO] Richtiges Handling
                            };

                            self.next_move_from = None;
                        };
                    }
                }
            }
        });
        // render the toolbar

        egui::Panel::top("toolbar")
            .resizable(false)
            .min_size(32.0)
            .show_inside(ui, |ui| {
                ui.visuals_mut().button_frame = false;

                ui.horizontal_centered(|ui| {
                    let file_response = ui.button(" File ");
                    egui::Popup::menu(&file_response)
                        .id(egui::Id::new("file_menu"))
                        .show(|ui| {
                            if ui.button("New").clicked() {
                                self.moves.clear_moves();
                                self.moves.state = STARTING_POSITION.to_string();
                            }
                        });

                    let board_response = ui.button("Board");
                    egui::Popup::menu(&board_response)
                        .id(egui::Id::new("board_menu"))
                        .show(|ui| {
                            if ui.button("Pieces").clicked() {
                                todo!();
                            }
                        });

                    let training_response = ui.button("Training");
                    egui::Popup::menu(&training_response)
                        .id(egui::Id::new("training_menu"))
                        .show(|ui| {
                            if ui.button("Games").clicked() {
                                todo!();
                            }
                        });

                    let analysis_response = ui.button("Analysis");
                    egui::Popup::menu(&analysis_response)
                        .id(egui::Id::new("analysis_menu"))
                        .show(|ui| {
                            if ui.button("Engine").clicked() {
                                todo!();
                            }
                        });
                });
            });

        // render the tools panel

        egui::Panel::right("tools")
            .resizable(true)
            .default_size(500.0)
            .max_size(500.0)
            .show_inside(ui, |ui| {
                ui.take_available_space();

                egui::Panel::top("top_tools")
                    .resizable(true)
                    .default_size(ui.available_height() * 0.7)
                    .show_inside(ui, |ui| {
                        ui.take_available_height();
                        ui.label("PGN");
                        ui.label(self.moves.get_pgn());
                    });

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.label("Analysis");
                });
            });

        // render the chessboard

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(
                    (ui.available_width() / 15.0).round() as i8,
                    0,
                ))
                .show(ui, |ui| {
                    egui::Grid::new(Id::new("grid_board"))
                        .spacing(Vec2::ZERO)
                        .show(ui, |ui| {
                            for i in 1..9 {
                                for j in 1..9 {
                                    let mut color = self.white_color;
                                    if (i % 2 == 0 && j % 2 != 0) || (i % 2 != 0 && j % 2 == 0) {
                                        color = self.black_color;
                                    }
                                    egui::Frame::NONE
                                        .fill(color)
                                        //.stroke(Stroke::new(1.0, Color32::GRAY))
                                        .show(ui, |ui| {
                                            self.draw_piece(
                                                ui,
                                                Square(j, i), // Unsafe if BoardSize changes
                                            );
                                        });
                                }
                                ui.end_row();
                            }
                            self.board_rect = ui.min_rect();
                        });
                });
            ui.label(self.to_move.to_string() + " to move");
        });
    }
}
