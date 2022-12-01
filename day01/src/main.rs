//! Solutions to day 01 puzzles
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").replace("\r\n", "\n");
    let summary = summarize(input);
    // println!("{:?}", summary);

    // get the max, then unwrap it from the Option.
    let max = (&summary).iter().max().unwrap();
    println!("The Elf with the most calories carries: {}", max);

    let top = top_three(summary);
    let sum: u32 = top.iter().sum();
    println!("The top 3 are {:?} and carry combined {} calories", top, sum);
}

fn summarize(input: String) -> Vec<u32> {
    let mut summary: Vec<u32> = vec![];

    for elf in input.split("\n\n") {
        // split elf string on new lines to iterator, convert to int, and sum
        let calories = elf.lines()
            .map(|n| { n.parse::<u32>().unwrap() })
            .sum();

        summary.push(calories);
    }

    return summary;
}

fn top_three(summary: Vec<u32>) -> Vec<u32> {
    // iter() creates iterator over references. -> Vec<&u32>
    // into_iter() allows you to iterate with ownership. -> Vec<u32>
    let sorted: Vec<u32> = summary.into_iter().sorted().rev().collect();

    return sorted[..3].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_should_sum_per_elf() {
        let input = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");

        let expected = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(summarize(input), expected);
    }

    #[test]
    fn top_three_should_return_three_highest() {
        let values = vec![3, 4, 4, 5, 6];

        assert_eq!(top_three(values), [6, 5, 4]);
    }
}