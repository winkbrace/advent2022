//! Opponent:       Player:         shape_score:
//! A = Rock        X = Rock        1
//! B = Paper       Y = Paper       2
//! C = Scissors    Z = Scissors    3
use std::collections::HashMap;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;
// delta: equal = 0, win = 1, loss = 2
// score: equal = 3, win = 6, loss = 0
const SCORES: [u8; 3] = [3, 6, 0];

fn main() {
    let input = include_str!("input.txt").replace("\r\n", "\n");
    let score = solve_part_one(input);

    println!("The total score for given input will be: {}", score);
}

// TODO split file to a part_one and part_two, then solve part two.
fn solve_part_one(input: String) -> u32 {
    return input.lines()
        .map(|line| parse_line(line))
        .map(|(opponent, player)| score(opponent, player))
        .sum();
}

fn parse_line(line: &str) -> (u8, u8) {
    let input_map: HashMap<char, u8> = HashMap::from([
        ('A', ROCK),
        ('B', PAPER),
        ('C', SCISSORS),
        ('X', ROCK),
        ('Y', PAPER),
        ('Z', SCISSORS),
    ]);

    let chars: Vec<char> = line.chars().collect();
    return (
        input_map.get(&chars[0]).unwrap().clone(),
        input_map.get(&chars[2]).unwrap().clone()
    );
}

fn score(opponent: u8, player: u8) -> u32 {
    return (player + score_round(opponent, player)).into();
}

fn score_round(opponent: u8, player: u8) -> u8 {
    let delta: usize = ((player + 3 - opponent) % 3).into();
    //println!("player {} - opponent {} = {}", player, opponent, delta);

    return SCORES[delta];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_round() {
        assert_eq!(score_round(ROCK, PAPER), 6);
        assert_eq!(score_round(PAPER, SCISSORS), 6);
        assert_eq!(score_round(SCISSORS, ROCK), 6);

        assert_eq!(score_round(ROCK, ROCK), 3);
        assert_eq!(score_round(PAPER, PAPER), 3);
        assert_eq!(score_round(SCISSORS, SCISSORS), 3);

        assert_eq!(score_round(ROCK, SCISSORS), 0);
        assert_eq!(score_round(PAPER, ROCK), 0);
        assert_eq!(score_round(SCISSORS, PAPER), 0);
    }

    #[test]
    fn test_score() {
        assert_eq!(score(ROCK, PAPER), 8);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("B X"), (PAPER, ROCK));
    }

    #[test]
    fn test_total_score_of_example_input() {
        let input = String::from("A Y
B X
C Z");
        assert_eq!(solve_part_one(input), 15);
    }
}