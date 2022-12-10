//! Opponent:       Player:         shape_score:
//! A = Rock        X = Rock        1
//! B = Paper       Y = Paper       2
//! C = Scissors    Z = Scissors    3

use std::collections::HashMap;

pub fn parse_line(line: &str) -> (u8, u8) {
    let input_map: HashMap<char, u8> = HashMap::from([
        ('A', crate::ROCK),
        ('B', crate::PAPER),
        ('C', crate::SCISSORS),
        ('X', crate::ROCK),
        ('Y', crate::PAPER),
        ('Z', crate::SCISSORS),
    ]);

    let chars: Vec<char> = line.chars().collect();
    return (
        input_map.get(&chars[0]).unwrap().clone(),
        input_map.get(&chars[2]).unwrap().clone()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("B X"), (crate::PAPER, crate::ROCK));
    }
}