use cozy_chess::*;
use std::io::stdin;
use std::str::FromStr;
use vampirc_uci::*;

use crate::ucisettings::UciSettings;
use std::io::{self, BufRead};
use vampirc_uci::{parse_one, UciMessage};

pub fn uci_init() {
    let mut uci = UciSettings::new();

    for line in io::stdin().lock().lines() {
        let msg: UciMessage = parse_one(&line.unwrap());
        match msg {
            UciMessage::Uci => {
                uci.handle_uci();
            }
            UciMessage::IsReady => {
                uci.handle_isready();
            }

            UciMessage::UciNewGame => {
                uci.new_game();
            }

            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                uci.handle_uci_position(startpos, fen, moves);
            }

            UciMessage::Go {
                time_control,
                search_control,
            } => {
                uci.handle_go_();
            }

            UciMessage::SetOption { name, value } => {
                if name == "Depth".to_string() {
                    if value.is_some() {
                        uci.handle_depth(value.unwrap().parse().expect("ERROR"));
                    }
                }
            }

            UciMessage::Quit => {
                uci.handle_quit();
            }

            UciMessage::Stop => {
                uci.handle_stop();
            }
            _ => {}
        }
    }
}
/*

// ENGINE: identify
id name Chess Engine
id author John Smith

// ENGINE: send the options that can be changed
//         in this case the hash size can have a value from 1 to 128 MB
option name Hash type spin default 1 min 1 max 128

// ENGINE: sent all parameters and is ready
uciok

// GUI: set hash to 32 MB
setoption name Hash value 32

// GUI: waiting for the engine to finish initializing
isready

// ENGINE: finished setting up the internal values and is ready to start
readyok

// GUI: let the engine know if starting a new game
ucinewgame

// GUI: tell the engine the position to search
position startpos moves e2e4

// GUI: tell the engine to start searching
//      in this case give it the timing information in milliseconds
go wtime 122000 btime 120000 winc 2000 binc 2000

// ENGINE: send search information continuously during search
//         this includes depth, search value, time, nodes, speed, and pv line
info depth 1 score cp -1 time 10 nodes 26 nps 633 pv e7e6
info depth 2 score cp -38 time 22 nodes 132 nps 2659 pv e7e6 e2e4
info depth 3 score cp -6 time 31 nodes 533 nps 10690 pv d7d5 e2e3 e7e6
info depth 4 score cp -30 time 55 nodes 1292 nps 25606 pv d7d5 e2e3 e7e6 g1f3

// ENGINE: return the best move found
bestmove d7d5

*/
