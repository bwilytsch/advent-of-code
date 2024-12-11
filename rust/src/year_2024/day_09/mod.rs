use std::collections::HashMap;

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

// Check the gaussian solution
pub fn part_two(input: &str) -> Result<u64> {
    let mut files: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut blanks: Vec<(u64, u64)> = vec![];

    let mut fid: u64 = 0;
    let mut pos: u64 = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let x = c
            .to_string()
            .parse::<u64>()
            .expect("Non-digit character encountered") as u64;
        if i % 2 == 0 {
            files.insert(fid, (pos, x));
            fid += 1;
        } else {
            if x != 0 {
                blanks.push((pos, x));
            }
        }
        pos += x;
    }

    while fid > 0 {
        fid -= 1;

        let (file_pos, size) = files[&fid];

        for i in 0..blanks.len() {
            let (start, length) = blanks[i];
            if start >= file_pos {
                blanks.truncate(i);
                break;
            }

            if size <= length as u64 {
                files.insert(fid, (start, size));
                if size == length as u64 {
                    blanks.remove(i);
                } else {
                    blanks[i] = (start + size, length - size);
                }
                break;
            }
        }
    }

    let mut checksum = 0;
    for (fid, (pos, size)) in files.iter() {
        for x in *pos..(*pos + *size) {
            checksum += fid * x;
        }
    }

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/009/example.txt") {
            assert_eq!(part_one(&input)?, 1928);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/009/input.txt") {
            println!("{}", part_one(&input)?);
        };

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/009/example.txt") {
            assert_eq!(part_two(&input)?, 2858);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/009/input.txt") {
            assert_eq!(part_two(&input)?, 6301361958738);
        }

        Ok(())
    }
}
