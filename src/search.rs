use crate::evalaute::Eval;
use crate::odering::{move_odering, KillerMoves};
use crate::tt::TranspositionTable;
use cozy_chess::*;
use std::time::Instant;

pub const INFINITY: i64 = 9999999;
pub const DEFAULT_MOVE: Move = Move {
    from: (Square::A1),
    to: (Square::A1),
    promotion: (None),
};

#[derive(Clone)]
pub struct Search {
    pub curr_score: i64,
    pub nodes: u64,
    pub tt: TranspositionTable,
    pub eval: Eval,
    pub killer: KillerMoves,
    pub depth: u64,
    pub start_time: u128,
    pub end_time: u128,
    pub max_depth: u64,
    pub max_nodes: u64,
    pub pv: Vec<Move>,
}

impl Search {
    pub fn new() -> Self {
        Search {
            curr_score: 0,
            nodes: 1,
            tt: TranspositionTable::new(),
            eval: Eval::new(),
            killer: KillerMoves::new(),
            depth: 0,
            start_time: 1,
            end_time: 1,
            max_depth: 100,
            max_nodes: u64::MAX,
            pv: Vec::new(),
        }
    }

    pub fn negamax(
        &mut self,
        board: &mut Board,
        depth: u64,
        mut alpha: i64,
        beta: i64,
        time: &Instant,
        hault: &bool,
    ) -> (i64, Move) {
        self.nodes += 1;
        self.pv = Vec::new();

        if let Some((score, halpha, hbeta, mv)) = self.tt.get(board.hash()) {
            if alpha == halpha && beta == hbeta {
                return (score, mv);
            }
        }

        let mut best_score = -INFINITY;
        let moves_list = move_odering(board, depth, &self.killer, &self.tt);
        let mut best_move = if moves_list.len() == 0 {
            DEFAULT_MOVE
        } else {
            moves_list[0].0
        };

        let board_status = board.status();

        if depth == 0
            || board_status != GameStatus::Ongoing
            || time.elapsed().as_millis() > self.end_time
            || *hault
            || self.nodes > self.max_nodes
            || self.depth > self.max_depth
        {
            if board_status == GameStatus::Won {
                if board.side_to_move() == Color::Black {
                    return (INFINITY, best_move);
                } else {
                    return (-INFINITY, best_move);
                }
            } else if board_status == GameStatus::Drawn {
                return (0, best_move);
            }
            self.pv = Vec::new();
            let fen = &board.to_string();
            let evaluation = self.eval.nnue_eval(fen);
            return (evaluation, best_move);
        }

        for (mv, _) in moves_list {
            let temp_board = board.clone();
            board.play(mv); //Make Move;
            self.curr_score = -self.negamax(board, depth - 1, -beta, -alpha, time, hault).0;
            *board = temp_board; // Unodo Move

            if self.curr_score > best_score {
                best_move = mv;
                best_score = best_score.max(self.curr_score);
            }

            if best_score > alpha {
                alpha = best_score
            }

            if alpha >= beta {
                self.killer.set_killer_move(depth as usize, mv);
                break;
            }

            if best_score < alpha {
                best_score = self.quisearch(board, 1, alpha, beta, time, hault);
            }
        }

        self.tt
            .insert(board.hash(), (best_score, alpha, beta, best_move));

        if self.depth == depth {
            self.pv.push(best_move);
            let pv_lines = self
                .pv
                .iter()
                .map(|mv| mv.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!(
                "info depth {} score cp {} time {} nodes {} nps {} pv {}",
                self.depth,
                best_score,
                time.elapsed().as_secs_f64() as u64,
                self.nodes,
                (self.nodes as f64 / time.elapsed().as_secs_f64()) as u64,
                pv_lines
            );
        }
        return (best_score, best_move);
    }

    pub fn quisearch(
        &mut self,
        board: &mut Board,
        depth: u64,
        mut alpha: i64,
        beta: i64,
        time: &Instant,
        hault: &bool,
    ) -> i64 {
        self.nodes += 1;
        if let Some((score, halpha, hbeta, _mv)) = self.tt.get(board.hash()) {
            if alpha == halpha && beta == hbeta {
                return score;
            }
        }

        let enemy_pieces = board.colors(!board.side_to_move());
        let mut cap_moves = Vec::new();

        board.generate_moves(|moves| {
            let mut attacks = moves.clone();
            attacks.to &= enemy_pieces;
            cap_moves.extend(moves);
            false
        });

        let board_status = board.status();
        let fen = &board.to_string();
        let evaluation = self.eval.nnue_eval(fen);

        if evaluation >= beta {
            return beta;
        };

        if alpha < evaluation {
            alpha = evaluation
        };

        if cap_moves.len() == 0
            || depth == 0
            || board_status != GameStatus::Ongoing
            || time.elapsed().as_millis() > self.end_time
            || *hault
            || self.nodes > self.max_nodes
            || self.depth > self.max_depth
        {
            if board_status == GameStatus::Won {
                if board.side_to_move() == Color::Black {
                    return INFINITY;
                } else {
                    return -INFINITY;
                }
            } else if board_status == GameStatus::Drawn {
                return 0;
            }
            return evaluation;
        }

        for mv in cap_moves {
            let temp_board = board.clone();
            board.play(mv); //Make Move;
            let score = -self.quisearch(board, depth - 1, -beta, -alpha, time, hault);
            *board = temp_board; // Unodo Move

            if score >= beta {
                return beta;
            };
            if score > alpha {
                alpha = score
            };
        }
        return alpha;
    }

    pub fn iter_deepning(
        &mut self,
        board: &mut Board,
        depth: Option<u64>,
        nodes: Option<u64>,
        time: Option<u128>,
        hault: &bool,
    ) -> (i64, Move) {
        let start_time = Instant::now();
        let color = board.side_to_move();
        self.max_nodes = if nodes.is_some() {
            nodes.unwrap()
        } else {
            u64::MAX
        };
        self.max_depth = if depth.is_some() { depth.unwrap() } else { 100 };
        self.end_time = if time.is_some() {
            time.unwrap()
        } else {
            u128::MAX
        };
        self.nodes = 1;
        self.depth = 1;

        let mut best_move = DEFAULT_MOVE;
        let mut best_score = if color == Color::Black {
            INFINITY
        } else {
            -INFINITY
        };

        for d in 1..self.max_depth {
            if self.nodes > self.max_nodes
                || start_time.elapsed().as_millis() > self.end_time
                || *hault
            {
                break;
            };
            let search2 = self.negamax(board, d, -INFINITY, INFINITY, &start_time, hault);
            self.depth += 1;

            if color == Color::Black && best_score >= search2.0 {
                best_score = search2.0;
                best_move = search2.1;
            } else if color == Color::White && best_score <= search2.0 {
                best_score = search2.0;
                best_move = search2.1;
            }
        }
        return (best_score, best_move);
    }
}
