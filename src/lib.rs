use data_processor::SymbolicMap;
use rand::seq::{IndexedRandom, IteratorRandom};

pub mod data_processor;
pub mod string_helper;

pub fn generate(words: &str, symbolic_map: &SymbolicMap, max_characters: usize) -> String {
    let mut rng = rand::rng();

    let mut result: String = words
        .chars()
        .filter_map(|c| {
            if let Some(values) = symbolic_map.get(&c) {
                values.choose(&mut rng)
            } else {
                None
            }
        })
        .collect();
    let mut last_char = words
        .chars()
        .last()
        .unwrap_or(*symbolic_map.keys().choose(&mut rng).unwrap_or(&' '));
    while let Some(values) = symbolic_map.get(&last_char)
        && !values.is_empty()
        && max_characters > 0
        && result.chars().count() < max_characters
    {
        last_char = *values.choose(&mut rng).unwrap();
        result.push(last_char);
    }

    result.trim().to_string()
}
