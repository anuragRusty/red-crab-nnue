use std::str::FromStr;

use crate::{odering::KillerMoves, search::*};
use cozy_chess::*;
use vampirc_uci::*;

const AUTHOR_NAME: &str = "Anurag Singh";
const ENGINE_NAME: &str = "RedCrab NNUE";
const VERSION: &str = "0.1";

pub struct UciSettings {
    pub depth: u64,
    pub nodes: u64,
    pub move_time: u128,
    pub board: Board,
    pub stop: bool,
    pub quit: bool,
    pub search: Search,
}

impl UciSettings {
    pub fn new() -> Self {
        UciSettings {
            depth: 10,
            nodes: 500000,
            move_time: 3000,
            board: Board::default(),
            stop: false,
            quit: false,
            search: Search::new(),
        }
    }

    pub fn handle_depth(&mut self, n: u64) {
        self.depth = n;
    }

    pub fn new_game(&mut self) {
        self.board = Board::default();
        self.search.killer = KillerMoves::new();
    }

    pub fn handle_uci(&mut self) {
        println!("id name {} {}", ENGINE_NAME, VERSION);
        println!("id author {}", AUTHOR_NAME);
        println!("");
        println!("option name Depth type spin default 6 min 1 max 100");
        println!("uciok");
    }

    pub fn handle_uci_position(
        &mut self,
        startpos: bool,
        fen: Option<UciFen>,
        moves: Vec<UciMove>,
    ) {
        if startpos {
            self.new_game();
        } else {
            self.board = Board::from_str(fen.unwrap().as_str()).unwrap();
            for mv in moves {
                self.board.play(Move::from_str(&mv.to_string()).unwrap());
            }
        }
    }

    pub fn handle_isready(&self) {
        println!("readyok");
    }

    pub fn handle_stop(&mut self) {
        self.stop = !self.stop;
    }

    pub fn handle_quit(&mut self) {
        self.quit = true;
    }

    pub fn handle_go_(&mut self) {
        let search =
            self.search
                .iter_deepning(&mut self.board, Some(self.depth), None, None, &self.stop);
        println!("bestmove {}", search.1);
    }
}
