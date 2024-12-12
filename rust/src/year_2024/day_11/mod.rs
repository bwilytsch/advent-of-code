use anyhow::Result;

use std::collections::HashMap;

fn blink(memo: &mut HashMap<(usize, usize), usize>, stone: usize, remaining: usize) -> usize {
    if remaining == 0 {
        return 1;
    }

    if let Some(&result) = memo.get(&(stone, remaining)) {
        return result;
    };

    let s = stone.to_string();
    if stone == 0 {
        let result = blink(memo, 1, remaining - 1);
        memo.insert((stone, remaining), result);
        result
    } else if s.len() % 2 == 0 {
        let half = s.len() / 2;
        let left_half = s[0..half].parse::<usize>().unwrap();
        let right_half = s[half..].parse::<usize>().unwrap();

        let result = blink(memo, left_half, remaining - 1) + blink(memo, right_half, remaining - 1);
        memo.insert((stone, remaining), result);
        result
    } else {
        let result = blink(memo, stone * 2024, remaining - 1);
        memo.insert((stone, remaining), result);
        result
    }
}

pub fn part_one(input: &str, blinks: usize) -> Result<usize> {
    // <(value, blinks), stones>
    let stones = input
        .split(" ")
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let result = stones.iter().map(|s| blink(&mut cache, *s, blinks)).sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() -> Result<()> {
        assert_eq!(part_one("125 17", 6)?, 22);
        assert_eq!(part_one("125 17", 25)?, 55312);

        println!("{}", part_one("5688 62084 2 3248809 179 79 0 172169", 25)?);

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        println!("{}", part_one("5688 62084 2 3248809 179 79 0 172169", 75)?);

        Ok(())
    }
}
