use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").replace("\r\n", "\n");

    let score = solve_part_one(&input);
    println!("The total priority score for part one is: {}", score);

    let score = solve_part_two(&input);
    println!("The total priority score for part two is: {}", score);
}

fn solve_part_one(input: &String) -> u32 {
    return input.lines()
        .map(|line| common_byte_in_half_lines(String::from(line)))
        .map(|byte| priority(byte))
        .sum();
}

fn solve_part_two(input: &String) -> u32 {
    return input.lines()
        .batching(|iterator| {
            let line = iterator.next();

            // stop iterator
            if line.is_none() {
                return None;
            }

            return Some((
                String::from(line.unwrap()),
                String::from(iterator.next().unwrap()),
                String::from(iterator.next().unwrap()),
            ))
        })
        .map(|set| common_byte_in_set(set))
        .map(|byte| priority(byte))
        .sum();
}

fn common_byte_in_half_lines(line: String) -> u8 {
    let bytes = line.as_bytes();
    let half_length = bytes.len() / 2;

    for head in &bytes[..half_length] {
        for tail in &bytes[half_length..] {
            if head == tail {
                return *head;
            }
        }
    }

    return 0;
}

fn common_byte_in_set(set: (String, String, String)) -> u8 {
    for i in set.0.as_bytes() {
        for j in set.1.as_bytes() {
            // Only check 3rd string if we have an equality in the first two.
            if i == j {
                for k in set.2.as_bytes() {
                    if i == k {
                        return *i;
                    }
                }
            }
        }
    }

    return 0;
}

// Get "priority" score per byte representation of a letter.
// ASCII byte codes: a-z are in range 97-122, A-Z in 65-90.
// The "priority" scores are: a-z are 1 - 26 and A-Z are 27-52.
fn priority(byte: u8) -> u32 {
    let prio = if byte <= 90 { byte - 38 } else { byte - 96 };

    return prio.into();
}

fn priority_from_letter(letter: char) -> u32 {
    return letter.to_digit(26).unwrap() - 9
        + if letter.is_uppercase() { 26 } else { 0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explore_letter_to_number() {
        // to_digit is built for hexadecimal type of digits. So 0 - 9, then a - z for base 36 :)
        // Base 36 is the highest allowed. Beware that this is case insensitive.
        assert_eq!('a'.to_digit(36).unwrap(), 10);
        assert_eq!('b'.to_digit(36).unwrap(), 11);
        assert_eq!('A'.to_digit(36).unwrap(), 10);
        assert_eq!('Z'.to_digit(36).unwrap(), 35);
    }

    #[test]
    fn explore_string_as_bytes() {
        // This is better, because it's part of Rust language
        assert_eq!(&[97, 98, 65, 90], "abAZ".as_bytes());
    }

    #[test]
    fn test_priority_from_letter() {
        assert_eq!(priority_from_letter('p'), 16);
        assert_eq!(priority_from_letter('P'), 42);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority(112), 16); // 'p'
        assert_eq!(priority(80), 42);  // 'P'
    }

    #[test]
    fn test_common_byte() {
        assert_eq!(112, common_byte_in_half_lines(String::from("vJrwpWtwJgWrhcsFMMfFFhFp")));
    }

    #[test]
    fn test_solve_part_one_example_input() {
        let input = include_str!("example.txt").replace("\r\n", "\n");
        assert_eq!(157, solve_part_one(&input));
    }

    #[test]
    fn test_common_byte_in_set() {
        let set = (
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        );
        assert_eq!(114, common_byte_in_set(set)); // 'r'
    }

    #[test]
    fn test_solve_part_two_example_input() {
        let input = include_str!("example.txt").replace("\r\n", "\n");
        assert_eq!(70, solve_part_two(&input));
    }
}