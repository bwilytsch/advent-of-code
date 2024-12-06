use std::collections::{HashMap, HashSet};

use anyhow::Result;

fn bubble_sort(pages: &Vec<&str>, rules: &HashMap<&str, HashSet<&str>>) -> Option<i32> {
    let mut v = pages.clone();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..v.len() - 1 {
            let a = v[i];
            let b = v[i + 1];

            if let Some(rule) = rules.get(b) {
                if rule.contains(a) {
                    v.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }

    if &v == pages {
        return None;
    }

    let middle = v[v.len() / 2];
    middle.parse::<i32>().ok()
}

fn is_valid(pages: &Vec<&str>, rules: &HashMap<&str, HashSet<&str>>) -> bool {
    for (i, p1) in pages.iter().enumerate() {
        for j in 0..i {
            if i == j {
                continue;
            }

            let Some(rule) = rules.get(p1) else {
                continue;
            };

            let compare = pages.get(j).unwrap();

            if rule.contains(compare) {
                return false;
            }
        }
    }

    true
}

/// This is a very primitive approach
pub fn part_one(input: &str) -> Result<i32> {
    let instructions = input.split("\n\n").collect::<Vec<&str>>();
    let (rules, pages) = (instructions[0], instructions[1]);

    let rules: HashMap<&str, HashSet<&str>> =
        rules.lines().fold(HashMap::new(), |mut map, line| {
            let mut p = line.split('|');

            let key = p.next().unwrap();
            let value = p.next().unwrap();

            let record = map.entry(key).or_insert({
                let mut set = HashSet::new();
                set.insert(value);
                set
            });

            record.insert(value);

            map
        });

    let sum = pages
        .lines()
        .filter_map(|line| {
            let pages = line.split(",").collect::<Vec<&str>>();

            if is_valid(&pages, &rules) {
                let middle = pages[pages.len() / 2];
                return middle.parse::<i32>().ok();
            }

            None
        })
        .sum::<i32>();

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<i32> {
    let instructions = input.split("\n\n").collect::<Vec<&str>>();
    let (rules, pages) = (instructions[0], instructions[1]);

    let rules: HashMap<&str, HashSet<&str>> =
        rules.lines().fold(HashMap::new(), |mut map, line| {
            let mut p = line.split('|');

            let key = p.next().unwrap();
            let value = p.next().unwrap();

            let record = map.entry(key).or_insert({
                let mut set = HashSet::new();
                set.insert(value);
                set
            });

            record.insert(value);

            map
        });

    let sum = pages
        .lines()
        .filter_map(|line| {
            let pages = line.split(",").collect::<Vec<&str>>();
            bubble_sort(&pages, &rules)
        })
        .sum::<i32>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/005/example.txt") {
            assert_eq!(part_one(&input)?, 143);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/005/input.txt") {
            println!("{}", part_one(&input)?);
        };

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/005/example.txt") {
            assert_eq!(part_two(&input)?, 123);
        };

        // 7730 too high
        if let Ok(input) = fs::read_to_string("./inputs/2024/005/input.txt") {
            println!("{}", part_two(&input)?);
        };

        Ok(())
    }
}
