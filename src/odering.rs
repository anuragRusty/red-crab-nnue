use crate::{
    search::{DEFAULT_MOVE, INFINITY},
    tt::TranspositionTable,
};
use cozy_chess::*;

const MVV_LVA: [[i64; 6]; 6] = [
    [100, 90, 80, 70, 60, 50],      // Pawn
    [200, 190, 180, 170, 160, 150], // Knight
    [300, 290, 280, 270, 260, 250], // Bishop
    [400, 390, 380, 370, 360, 350], // Rook
    [500, 490, 480, 470, 460, 450], // Queen
    [600, 590, 580, 570, 560, 550], // King
];

const KILLER_VALUE: i64 = 10;
const MAX_DEPTH: usize = 128;
const TTMOVE_VALUE: i64 = 60;

fn get_piece_value(piece: Piece) -> i64 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 2000,
    }
}

#[derive(Clone)]
pub struct KillerMoves {
    pub moves: [[Move; 2]; MAX_DEPTH],
}

impl KillerMoves {
    pub fn new() -> Self {
        let moves = [[DEFAULT_MOVE; 2]; MAX_DEPTH];
        Self { moves }
    }

    pub fn set_killer_move(&mut self, depth: usize, mv: Move) {
        if self.moves[depth][0] == mv {
            return;
        } else {
            self.moves[depth][1] = self.moves[depth][0];
            self.moves[depth][0] = mv;
        }
    }
    pub fn get_killer_moves(&self, depth: usize) -> [Move; 2] {
        self.moves[depth]
    }
}

fn mvv_lva_score(attacker: Piece, victim: Piece) -> i64 {
    MVV_LVA[attacker as usize][victim as usize]
}

pub fn move_odering(
    board: &mut Board,
    depth: u64,
    km: &KillerMoves,
    tt: &TranspositionTable,
) -> Vec<(Move, i64)> {
    let mut moves_list = Vec::new();
    let mut tt_move = DEFAULT_MOVE;
    //let mut null_board = board.clone().null_move();
    //let mut pawn_attacks = BitBoard::EMPTY;

    if let Some((_score, _halpha, _hbeta, mv)) = tt.get(board.hash()) {
        tt_move = mv;
    }
    board.generate_moves(|moves| {
        for mv in moves {
            let moving_piece = board.piece_on(mv.from).unwrap();
            let capturing_piece = board.piece_on(mv.to);

            if tt_move == mv {
                moves_list.push((mv, TTMOVE_VALUE));
            } else if capturing_piece.is_some() {
                let score = mvv_lva_score(moving_piece, capturing_piece.unwrap());
                moves_list.push((mv, score));
            } else if mv == km.get_killer_moves(depth as usize)[0] {
                moves_list.push((mv, KILLER_VALUE));
            } else if mv == km.get_killer_moves(depth as usize)[1] {
                moves_list.push((mv, KILLER_VALUE));
            } else if mv.promotion.is_some() {
                moves_list.push((mv, get_piece_value(mv.promotion.unwrap()) / 10));
            } else {
                moves_list.push((mv, 0));
            }
        }
        false
    });

    moves_list.sort_by(|a, b| b.1.cmp(&a.1));

    moves_list
}
