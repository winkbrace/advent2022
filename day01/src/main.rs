//! Solutions to day 01 puzzles
fn main() {
    println!("Hello, world!");
}

fn summarize(input: &str) -> Vec<i32> {
    let mut summary: Vec<i32> = vec![];

    for elf in input.split("\n\n") {
        // split elf string on new lines to iterator, convert to int, and sum
        let calories = elf.lines()
            .map(|n| { n.parse::<i32>().unwrap() })
            .sum();

        summary.push(calories);
    }

    return summary;
}

#[cfg(test)]
mod tests {
    use super::summarize;

    #[test]
    fn summarize_should_sum_per_elf() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let expected = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(summarize(&input), expected);
    }
}