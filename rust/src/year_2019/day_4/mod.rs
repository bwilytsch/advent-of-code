use anyhow::Result;

// Maybe bit shifting to find doubles works here
fn check(num: &str) -> bool {
    let mut prev = 0;
    let mut double = false;

    for n in num.chars().filter_map(|c| c.to_digit(10)) {
        if n < prev {
            return false;
        }

        if prev == n {
            double = true;
        }

        prev = n;
    }

    if !double {
        return false;
    }

    true
}

fn check_thorough(num: &i32) -> bool {
    let mut prev = 0;
    let mut doubled = false;
    let mut collector: Vec<u32> = vec![];

    // needs a pair, groups of 2 or more don't count

    for n in num.to_string().chars().filter_map(|c| c.to_digit(10)) {
        if n < prev {
            return false;
        }

        if let Some(a) = collector.last() {
            if a != &n {
                if collector.len() == 2 {
                    doubled = true;
                }

                collector.clear();
            }

            collector.push(n);
        } else {
            collector.push(n);
        }

        prev = n;
    }

    doubled || collector.len() == 2
}

fn part_one(input: &str) -> Result<i32> {
    let ranges = input
        .split('-')
        .map(|sub| sub.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;

    if let (Some(start), Some(end)) = (ranges.first(), ranges.get(1)) {
        for pw in *start..=*end {
            if check(&pw.to_string()) {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn part_two(input: &str) -> Result<i32> {
    let [start, end] = input
        .split('-')
        .map(|sub| sub.parse::<i32>().unwrap())
        .collect::<Vec<_>>()[..]
    // this last part creates a slice out of a Vec
    else {
        return Ok(0); // Refutable pattern
    };

    Ok((start..=end).filter(check_thorough).count() as i32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() -> Result<()> {
        assert!(check("111111"));
        assert!(!check("223450"));
        assert!(!check("123789"));

        println!("{}", part_one("347312-805915")?);

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        assert!(check_thorough(&112233));
        assert!(!check_thorough(&123444));
        assert!(check_thorough(&111122));

        println!("{}", part_two("347312-805915")?);

        Ok(())
    }
}
