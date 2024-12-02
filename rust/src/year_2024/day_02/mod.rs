use anyhow::Result;

pub fn part_one(input: &str) -> Result<i32> {
    let valid_reports = input
        .lines()
        .filter_map(|report| {
            let levels = report
                .split_whitespace()
                .filter_map(|level| level.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            let mut prev_dir = 0;

            if levels.windows(2).all(|v| {
                let a = v[0];
                let b = v[1];

                let mut delta = a - b;

                let current_dir = match delta < 0 {
                    true => -1,
                    false => 1,
                };

                if prev_dir != 0 && prev_dir != current_dir {
                    return false;
                };

                prev_dir = current_dir;

                delta = delta.abs();

                !(delta < 1 || delta > 3)
            }) {
                return Some(1);
            }

            None
        })
        .sum();

    Ok(valid_reports)
}

// pub fn part_two(input: &str) -> Result<i32> {
//     Ok(0)
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        let example = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part_one(example)?, 2);

        if let Ok(file_input) = fs::read_to_string("./inputs/2024/002/input.txt") {
            println!("{}", part_one(&file_input)?);
        }

        Ok(())
    }

    // #[test]
    // fn two() -> Result<()> {
    // }
}
