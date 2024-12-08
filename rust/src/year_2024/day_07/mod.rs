use anyhow::Result;

fn calc(values: &Vec<i64>, ops: &Vec<usize>) -> i64 {
    if values.len() < 2 {
        return 0;
    }

    let mut sum = values[0];

    for (i, op) in ops.iter().enumerate() {
        let v = values[i + 1];

        if *op == 0 {
            sum += v;
        } else {
            sum *= v;
        }
    }

    sum
}

pub fn part_one(input: &str) -> Result<i64> {
    let mut sum = 0;

    for line in input.lines() {
        if let Some((result, params)) = line.split_once(": ") {
            let values = params
                .split(" ")
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<i64>>();
            let result = result.parse::<i64>().expect("Couldn't get a result");

            let length = values.len() - 1;
            for i in 0..(1 << length) {
                let mut ops = vec![0; length];
                for j in 0..length {
                    ops[j] = (i >> j) & 1;
                }

                let calculated = calc(&values, &ops);
                if calculated == result {
                    sum += result;
                    break;
                }
            }
        }
    }

    Ok(sum)
}

// pub fn part_two(input: &str) -> Result<()> {
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/007/example.txt") {
            assert_eq!(part_one(&input)?, 3749);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/007/input.txt") {
            println!("{}", part_one(&input)?);
        };

        Ok(())
    }

    // #[test]
    // fn two() -> Result<()> {
    //     if let Ok(input) = fs::read_to_string("./inputs/2024/006/example.txt") {
    //         assert_eq!(part_two(&input)?, 6);
    //     };
    //
    //     if let Ok(input) = fs::read_to_string("./inputs/2024/006/input.txt") {
    //         println!("{}", part_two(&input)?);
    //     }
    //
    //     Ok(())
    // }
}
