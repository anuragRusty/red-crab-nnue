use binread::BinRead;
use nnue::stockfish::halfkp::{SfHalfKpFullModel, SfHalfKpModel};
use nnue::*;
use std::io::Cursor;

const NNUE: &[u8] = include_bytes!("nn.nnue");

#[derive(Clone)]
pub struct Eval {
    model2: SfHalfKpModel,
}

impl Eval {
    pub fn new() -> Self {
        let mut reader = Cursor::new(NNUE);
        let model = SfHalfKpFullModel::read(&mut reader).unwrap();
        let model2 = model.model;
        Eval {
            model2: model2,
        }
    }

    pub fn nnue_eval(&mut self, fen: &str) -> i64 {
        
        let mut parts = fen.split_ascii_whitespace();
        let pos = parts.next().unwrap();
        let mut pieces = Vec::new();
        for (rank, row) in pos.rsplit("/").enumerate() {
            let mut file = 0;
            for p in row.chars() {
                if let Some(offset) = p.to_digit(10) {
                    file += offset as usize;
                } else {
                    let piece = match p.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => panic!("Invalid piece {}", p),
                    };
                    let color = if p.is_ascii_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let square = Square::from_index(rank * 8 + file);
                    pieces.push((piece, color, square));
                    file += 1;
                }
            }
        }
        let side_to_move = if parts.next().unwrap() == "w" {
            Color::White
        } else {
            Color::Black
        };
        let mut white_king = Square::A1;
        let mut black_king = Square::A1;
        for &(piece, color, square) in &pieces {
            if piece == Piece::King {
                if color == Color::White {
                    white_king = square;
                } else {
                    black_king = square;
                }
            }
        }
        let mut state = self.model2.new_state(white_king, black_king);
        for &(piece, piece_color, square) in &pieces {
            if piece != Piece::King {
                for &color in &Color::ALL {
                    state.add(color, piece, piece_color, square);
                }
            }
        }
        (state.activate(side_to_move)[0] / 16) as i64
    }
}
