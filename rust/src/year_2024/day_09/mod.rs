use anyhow::Result;

pub fn part_one(input: &str) -> Result<u64> {
    let mut pointer = 0;
    let mut disk = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let n = c
                .to_string()
                .parse::<usize>()
                .expect("This is not a number");

            let mut r = vec![];

            if (i % 2) == 0 {
                for _ in 0..n {
                    r.push(pointer.to_string());
                }

                pointer += 1;

                return r;
            };

            for _ in 0..n {
                r.push(".".to_string());
            }

            r
        })
        .flatten()
        .collect::<Vec<String>>();

    let len = disk.len();

    let mut p0 = 0;
    let mut p1 = len - 1;

    loop {
        let mut a = disk[p0].clone();
        let mut b = disk[p1].clone();

        while a != "." {
            p0 += 1;
            a = disk[p0].clone();
        }

        while b == "." {
            p1 -= 1;
            b = disk[p1].clone();
        }

        if p1 < p0 {
            break;
        }

        disk.swap(p0, p1);
    }

    let mut sum = 0;

    for i in 0..len {
        let v = disk[i].clone();

        if v == "." {
            break;
        }

        sum += v.parse::<u64>().unwrap_or(0) * (i as u64);
    }

    Ok(sum)
}

// pub fn part_two(input: &str) -> Result<usize> {}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/009/example.txt") {
            assert_eq!(part_one(&input)?, 1928);
        };

        // 89813169309 - too low
        if let Ok(input) = fs::read_to_string("./inputs/2024/009/input.txt") {
            println!("{}", part_one(&input)?);
        };

        Ok(())
    }

    // #[test]
    // fn two() -> Result<()> {
    //     if let Ok(input) = fs::read_to_string("./inputs/2024/008/example.txt") {
    //         assert_eq!(part_two(&input)?, 34);
    //     };
    //
    //     if let Ok(input) = fs::read_to_string("./inputs/2024/008/input.txt") {
    //         println!("{}", part_two(&input)?);
    //     }
    //
    //     Ok(())
    // }
}
