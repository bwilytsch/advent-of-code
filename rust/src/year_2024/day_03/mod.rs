use anyhow::{anyhow, Result};
use regex::Regex;

fn sum_valid(input: &str) -> Result<i32> {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    Ok(sum)
}

pub fn part_one(input: &str) -> Result<i32> {
    sum_valid(input)
}

pub fn part_two(input: &str) -> Result<i32> {
    let mut chunks = input.split("don't()");
    let first = chunks.next().ok_or_else(|| anyhow!("Input is empty"))?;

    let value = sum_valid(first)?;

    let value_2: i32 = chunks
        .filter_map(|c| {
            let result = c.split("do()").skip(1).collect::<Vec<&str>>().concat();
            sum_valid(&result).ok()
        })
        .sum();

    Ok(value + value_2)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(example)?, 161);

        if let Ok(input) = fs::read_to_string("./inputs/2024/003/input.txt") {
            println!("{}", part_one(&input)?);
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(part_two(example)?, 48);

        if let Ok(input) = fs::read_to_string("./inputs/2024/003/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
