use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn get_points_with_steps(wire: &str) -> Result<(HashSet<Point>, HashMap<Point, i32>)> {
    let mut output = HashSet::new();
    let mut dist = HashMap::new();

    let mut length = 0;
    let mut x = 0;
    let mut y: i32 = 0;

    for instruction in wire.trim().split(',') {
        let (dir, steps) = (&instruction[0..1], &instruction[1..]);
        let s = steps.parse::<i32>()?;

        for _ in 0..s {
            match dir {
                "U" => {
                    y += 1;
                }
                "L" => {
                    x -= 1;
                }
                "R" => {
                    x += 1;
                }
                _ => {
                    y -= 1;
                }
            }

            length += 1;
            let p = Point { x, y };
            output.insert(p.clone());
            dist.insert(p, length);
        }
    }

    Ok((output, dist))
}

fn get_points(wire: &str) -> Result<HashSet<Point>> {
    let mut output = HashSet::new();
    let mut x = 0;
    let mut y: i32 = 0;

    for instruction in wire.trim().split(',') {
        let (dir, steps) = (&instruction[0..1], &instruction[1..]);
        let s = steps.parse::<i32>()?;

        for _ in 0..s {
            match dir {
                "U" => {
                    y += 1;
                }
                "L" => {
                    x -= 1;
                }
                "R" => {
                    x += 1;
                }
                _ => {
                    y -= 1;
                }
            }

            output.insert(Point { x, y });
        }
    }

    Ok(output)
}

pub fn part_one(input: &str) -> Result<i32> {
    if let (Some(first), Some(second)) = (input.lines().next(), input.lines().nth(1)) {
        let pa = get_points(first.trim())?;
        let pb = get_points(second.trim())?;

        let mut both = pa
            .intersection(&pb)
            .map(|p| p.x.abs() + p.y.abs())
            .collect::<Vec<_>>();

        both.sort();

        if let Some(smallest) = both.first() {
            return Ok(*smallest);
        }
    };

    Err(anyhow!("Nothing found"))
}

pub fn part_two(input: &str) -> Result<i32> {
    if let (Some(first), Some(second)) = (input.lines().next(), input.lines().nth(1)) {
        let (pa, ha) = get_points_with_steps(first.trim())?;
        let (pb, hb) = get_points_with_steps(second.trim())?;

        let both = pa.intersection(&pb).collect::<Vec<_>>();

        let mut steps = both
            .iter()
            .map(|x| ha.get(x).unwrap() + hb.get(x).unwrap())
            .collect::<Vec<i32>>();

        steps.sort();

        return Ok(steps[0]);
    };

    Err(anyhow!("Nothing found"))
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        assert_eq!(
            part_one(
                "R8,U5,L5,D3
                U7,R6,D4,L4"
            )?,
            6
        );

        assert_eq!(
            part_one(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83"
            )?,
            159
        );

        assert_eq!(
            part_one(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )?,
            135
        );

        let input = fs::read_to_string("./inputs/2019/003/input.txt")?;
        println!("{}", part_one(&input)?);

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        assert_eq!(
            part_two(
                "R8,U5,L5,D3
                U7,R6,D4,L4"
            )?,
            30
        );

        let input = fs::read_to_string("./inputs/2019/003/input.txt")?;
        println!("{}", part_two(&input)?);

        Ok(())
    }
}
