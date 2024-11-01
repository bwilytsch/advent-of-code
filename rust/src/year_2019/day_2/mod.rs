use core::panic;

use anyhow::Result;

fn calc(nums: &mut Vec<i32>) -> Result<String> {
    let mut cursor = 0;

    while cursor < nums.len() {
        let v = nums[cursor];

        match v {
            2 => {
                let a = nums[cursor + 1] as usize;
                let b = nums[cursor + 2] as usize;
                let position = nums[cursor + 3] as usize;

                nums[position] = nums[a] * nums[b];

                cursor += 4;
            }
            1 => {
                let a = nums[cursor + 1] as usize;
                let b = nums[cursor + 2] as usize;
                let position = nums[cursor + 3] as usize;

                nums[position] = nums[a] + nums[b];

                cursor += 4;
            }
            99 => {
                cursor = nums.len();
            }
            _ => {
                cursor += 1;
            }
        }
    }

    Ok((nums[0]).to_string())
}

pub fn part_one(input: &str) -> Result<String> {
    let mut nums = input
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    if nums.len() > 12 {
        nums[1] = 12;
        nums[2] = 2;
    }

    calc(&mut nums)
}

pub fn part_two(input: &str, expected: &str) -> Result<(i32, i32)> {
    let mut nums = input
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            nums[1] = noun;
            nums[2] = verb;

            let computed = calc(&mut nums.clone())?;

            if computed == expected {
                return Ok((noun, verb));
            }
        }
    }

    panic!("Nothing found")
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        let test_input = "
                1,9,10,3,
                2,3,11,0,
                99,
                30,40,50
            ";

        assert_eq!(part_one(test_input)?, "3500");
        assert_eq!(part_one("1,0,0,0,99")?, "2");
        assert_eq!(part_one("2,4,4,5,99,0")?, "2");
        assert_eq!(part_one("1,1,1,4,99,5,6,0,99")?, "30");

        let input = fs::read_to_string("./inputs/2019/002/part1.txt")?;

        println!("day one: {}", part_one(&input).unwrap());

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        let input = fs::read_to_string("./inputs/2019/002/part1.txt")?;

        println!("{:?}", part_two(&input, "19690720")?);

        Ok(())
    }
}
