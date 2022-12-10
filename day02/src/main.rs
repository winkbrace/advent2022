mod part_one;
mod part_two;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

// delta: equal = 0, win = 1, loss = 2
// score: equal = 3, win = 6, loss = 0
const SCORES: [u8; 3] = [3, 6, 0];

fn main() {
    let input = include_str!("input.txt").replace("\r\n", "\n");

    let score = solve(&input, part_one::parse_line);
    println!("The total score for part one will be: {}", score);

    let score = solve(&input, part_two::parse_line);
    println!("The total score for part two will be: {}", score);
}

pub fn solve(input: &String, parser: fn(&str) -> (u8, u8)) -> u32 {
    return input.lines()
        .map(|line| parser(line))
        .map(|(opponent, player)| score(opponent, player))
        .sum();
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
    fn test_total_score_of_example_input() {
        let input = String::from("A Y
B X
C Z");
        assert_eq!(solve(&input, part_one::parse_line), 15);
    }
}