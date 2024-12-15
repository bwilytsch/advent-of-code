use anyhow::Result;
use regex::Regex;

// Not every class machine is reachable
// For all reachable what's minimum amounts of tokens one has to spend?

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(isize, isize);
impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"X\=(\d+), Y\=(\d+)").unwrap();
        let manual = s.split(": ").collect::<Vec<&str>>();
        let config = manual.get(1).unwrap();

        if let Some(caps) = re.captures(*config) {
            let x = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();

            return Point(x, y);
        }

        panic!("This should not happen")
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Button(isize, isize, isize);

impl From<&str> for Button {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let manual = s.split(": ").collect::<Vec<&str>>();
        let button_type = manual
            .get(0)
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .to_lowercase();
        let config = manual.get(1).unwrap();

        let mut cost = 3;

        if button_type == "b" {
            cost = 1;
        }

        if let Some(caps) = re.captures(*config) {
            let x = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();

            return Button(x, y, cost);
        }

        panic!("This should not happen")
    }
}

fn solve_linear(machine: &(Vec<Button>, Point), offset: isize) -> Option<isize> {
    let (buttons, prize) = machine;

    let a_button = buttons[0];
    let b_button = buttons[1];

    let a1 = a_button.0;
    let a2 = a_button.1;
    let b1 = b_button.0;
    let b2 = b_button.1;
    let c1 = prize.0 + offset;
    let c2 = prize.1 + offset;

    // Solve for b.
    let num = a2 * c1 - a1 * c2;
    let denom = a2 * b1 - a1 * b2;

    // Check for "no solution". Obviously we can't divide by zero and the
    // division must be exact because you either press the button or you
    // don't.
    if denom == 0 {
        return None;
    } else if num % denom != 0 {
        return None;
    }
    let b = num / denom;

    // Solve for a.
    let num = c1 - b * b1;
    let denom = a1;
    if denom == 0 {
        return None;
    } else if num % denom != 0 {
        return None;
    }

    let a = num / denom;

    Some(a * 3 + b)
}

fn get_machines(input: &str) -> Vec<(Vec<Button>, Point)> {
    input
        .split("\n\n")
        .map(|m| {
            // Should be three lines
            let mut lines = m.lines();
            let a: Button = lines.next().unwrap().into();
            let b: Button = lines.next().unwrap().into();
            let prize: Point = lines.next().unwrap().into();

            return (vec![a, b], prize);
        })
        .collect::<Vec<(Vec<Button>, Point)>>()
}

pub fn part_one(input: &str) -> Result<isize> {
    let machines = get_machines(input);
    let tokens: isize = machines.iter().filter_map(|m| solve_linear(m, 0)).sum();

    Ok(tokens)
}

// Source: https://en.wikipedia.org/wiki/Cramer%27s_rule
pub fn part_two(input: &str) -> Result<isize> {
    let machines = get_machines(input);
    let tokens: isize = machines
        .iter()
        .filter_map(|m| solve_linear(m, 10000000000000))
        .sum();

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(example) = fs::read_to_string("./inputs/2024/013/example.txt") {
            assert_eq!(part_one(&example)?, 480);
        }

        if let Ok(input) = fs::read_to_string("./inputs/2024/013/input.txt") {
            println!("{}", part_one(&input)?);
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        // 42991637687773 -- too low
        if let Ok(input) = fs::read_to_string("./inputs/2024/013/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
