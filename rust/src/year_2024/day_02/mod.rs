use anyhow::Result;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|level| level.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

fn is_safe(levels: &Vec<i32>) -> Option<i32> {
    match levels.windows(2).all(|v| (1..=3).contains(&(v[0] - v[1])))
        || levels.windows(2).all(|v| (1..=3).contains(&(v[1] - v[0])))
    {
        true => Some(0),
        false => None,
    }
}

pub fn part_one(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(parse_input)
        .filter_map(|levels| is_safe(&levels))
        .count())
}

pub fn part_two(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .filter_map(|report| {
            let levels = parse_input(&report);

            if is_safe(&levels).is_some() {
                return Some(1);
            } else {
                // This is a brute force method
                match (0..levels.len())
                    .map(|i| {
                        levels
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, &value)| if i != idx { Some(value) } else { None })
                            .collect()
                    })
                    .any(|abridged_level| is_safe(&abridged_level).is_some())
                {
                    true => Some(1),
                    false => None,
                }
            }
        })
        .count())
}

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

    #[test]
    fn two() -> Result<()> {
        let example = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part_two(example)?, 4);

        if let Ok(file_input) = fs::read_to_string("./inputs/2024/002/input.txt") {
            println!("{}", part_two(&file_input)?);
        }

        Ok(())
    }
}
