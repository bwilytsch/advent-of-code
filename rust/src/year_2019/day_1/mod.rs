use std::{collections::HashMap, ops::Div};

use anyhow::Result;

fn get_fuel(mass: i32) -> Result<i32> {
    Ok(((mass as f32).div(3.0_f32) as i32) - 2)
}

pub fn part_one(input: &str) -> Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let value = line.trim().parse::<_>()?;
        let mass = get_fuel(value)?;
        sum += mass;
    }

    Ok(sum.to_string())
}

fn get_fuel_rec(mass: i32, cache: &HashMap<i32, i32>) -> i32 {
    if let Some(cached_value) = cache.get(&mass) {
        return *cached_value;
    }

    let fuel = ((mass as f32).div(3.0_f32) as i32) - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + get_fuel_rec(fuel, cache)
}

// This needs memoization
// This is currently brute forced
pub fn part_two(input: &str) -> Result<String> {
    let cache = HashMap::new();
    let mut sum = 0;

    for line in input.lines() {
        let start_mass = line.trim().parse::<_>()?;
        sum += get_fuel_rec(start_mass, &cache);
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        assert_eq!(part_one("1969")?, "654");
        assert_eq!(part_one("100756")?, "33583");

        let input = fs::read_to_string("./inputs/2019/001/part1.txt")?;

        println!("day one: {:?}", part_one(&input)?);

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        assert_eq!(part_two("1969")?, "966");
        assert_eq!(part_two("100756")?, "50346");

        let input = fs::read_to_string("./inputs/2019/001/part1.txt")?;

        println!("day two: {:?}", part_two(&input)?);

        Ok(())
    }
}
