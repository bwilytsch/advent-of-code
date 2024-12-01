use anyhow::Result;

pub fn part_one(input: &str) -> Result<i32> {
    let mut columns: (Vec<&str>, Vec<&str>) = input
        .lines()
        .map(|line| line.trim().split_once(' ').unwrap())
        .unzip();

    columns.0.sort();
    columns.1.sort();

    let mut sum = 0;

    // Assuming they are of equal length
    for (index, entry) in columns.0.iter().enumerate() {
        let value = entry.parse::<i32>().unwrap();
        let other = columns.1.get(index).unwrap().trim().parse::<i32>().unwrap();
        let delta = (value - other).abs();
        sum += delta;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        let test = "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3";

        assert_eq!(part_one(test)?, 11);

        if let Ok(file_input) = fs::read_to_string("./inputs/2024/001/input.txt") {
            println!("{}", part_one(&file_input)?);
        }

        Ok(())
    }

    // #[test]
    // fn two() -> Result<()> {
    //     todo!()
    // }
}
