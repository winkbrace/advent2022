//! Solutions to day 01 puzzles
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have read input.txt");
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

    for elf in input.replace("\r\n", "\n").split("\n\n") {
        // split elf string on new lines to iterator, convert to int, and sum
        let calories = elf.lines()
            .map(|n| { n.parse::<u32>().unwrap() })
            .sum();

        summary.push(calories);
    }

    return summary;
}

fn top_three(mut summary: Vec<u32>) -> Vec<u32> {
    // sort desc
    summary.sort_by(|a, b| b.cmp(a));

    return summary[..3].to_vec();
}

#[cfg(test)]
mod tests {
    use super::summarize;
    use super::top_three;

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

        assert_eq!(summarize(input.to_string()), expected);
    }

    #[test]
    fn top_three_should_return_three_highest() {
        let values = vec![3, 4, 4, 5, 6];

        assert_eq!(top_three(values), [6, 5, 4]);
    }
}