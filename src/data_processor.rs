use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

pub type SymbolicMap = HashMap<char, Vec<char>>;

#[derive(Serialize, Deserialize, Default)]
pub struct SymbolicData {
    hash: u64,
    pub data: SymbolicMap,
}

impl PartialEq for SymbolicData {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.data == other.data
    }
}

impl PartialEq<String> for SymbolicData {
    fn eq(&self, other: &String) -> bool {
        let mut hasher = DefaultHasher::new();
        other.hash(&mut hasher);
        self.hash == hasher.finish()
    }
}

impl SymbolicData {
    pub fn process_data(content: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        let mut data = SymbolicMap::new();

        content.hash(&mut hasher);

        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .for_each(|line| process_line(line, &mut data));

        Self {
            data,
            hash: hasher.finish(),
        }
    }
}

fn process_line(line: &str, symbolic_map: &mut SymbolicMap) {
    line.chars().fold(char::default(), |i, c| {
        if i == char::default() {
            return c;
        }

        symbolic_map
            .entry(i)
            .and_modify(|values| values.push(c))
            .or_insert(vec![c]);

        c
    });
}
