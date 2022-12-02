//! Opponent:       Player:         shape_score:
//! A = Rock        X = Rock        1
//! B = Paper       Y = Paper       2
//! C = Scissors    Z = Scissors    3

const ROCK: i8 = 1;
const PAPER: i8 = 2;
const SCISSORS: i8 = 3;

fn main() {
    //let input = include_str!("input.txt").replace("\r\n", "\n");
}

fn score(opponent: i8, player: i8) -> i8 {
    player + score_round(opponent, player);
}

fn score_round(opponent: i8, player: i8) -> i8 {
    let delta = player - opponent;
    println!("{delta}");
    match delta {
        -1|2 => return 0,
        0 => return 3,
        1|-2 => return 6
    }
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
}