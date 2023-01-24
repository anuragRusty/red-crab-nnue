use std::collections::HashMap;
use cozy_chess::Move;
#[derive(Clone)]
pub struct TranspositionTable {
    pub table: HashMap<u64, (i64, i64, i64,Move)>,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: u64) -> Option<(i64, i64, i64,Move)> {
        self.table.get(&key).cloned()
    }

    pub fn insert(&mut self, key: u64, value: (i64, i64, i64,Move)) {
        self.table.insert(key, value);
    }
}
