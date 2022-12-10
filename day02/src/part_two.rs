//! Opponent:       Player:         shape_score:
//! A = Rock        X = Lose        1
//! B = Paper       Y = Draw        2
//! C = Scissors    Z = Win         3

use std::collections::HashMap;

pub fn parse_line(line: &str) -> (u8, u8) {
    let input_map: HashMap<char, u8> = HashMap::from([
        ('A', crate::ROCK),
        ('B', crate::PAPER),
        ('C', crate::SCISSORS),
    ]);

    let chars: Vec<char> = line.chars().collect();
    let opponent = input_map.get(&chars[0]).unwrap().clone();
    return (
        opponent,
        get_hand_sign_by_result(opponent, chars[2])
    );
}

fn get_hand_sign_by_result(opponent: u8, result_char: char) -> u8 {
   let result = if result_char == 'X' { // Lose
        (opponent + 2) % 3
    } else if result_char == 'Y' {
        opponent
    } else {
        (opponent + 1) % 3
    };

    return if result == 0 { crate::SCISSORS } else { result };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hand_sign_by_result() {
        assert_eq!(get_hand_sign_by_result(crate::SCISSORS, 'X'), crate::PAPER);
        assert_eq!(get_hand_sign_by_result(crate::ROCK, 'Y'), crate::ROCK);
        assert_eq!(get_hand_sign_by_result(crate::PAPER, 'Z'), crate::SCISSORS);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("A Y"), (crate::ROCK, crate::ROCK));
        assert_eq!(parse_line("B X"), (crate::PAPER, crate::ROCK));
        assert_eq!(parse_line("C Z"), (crate::SCISSORS, crate::ROCK));
    }
}