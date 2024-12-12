use std::{cmp::Ordering, collections::HashSet, fmt::Display};

use anyhow::Result;

// This is a very bad and verbose attempt of this, should come back to this later

#[derive(Debug, Hash)]
struct Point(i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct Grid {
    rows: usize,
    columns: usize,
}

struct Direction(i32, i32);
const DIRS: [Direction; 4] = [
    Direction(0, -1),
    Direction(1, 0),
    Direction(0, 1),
    Direction(-1, 0),
];

fn move_to(p: &Point, dir: &Direction) -> Point {
    let x = p.0 + dir.0;
    let y = p.1 + dir.1;

    Point(x, y)
}

fn index_to_position(index: usize, grid: &Grid) -> Point {
    Point((index % grid.columns) as i32, (index / grid.columns) as i32)
}

fn position_to_index(p: &Point, grid: &Grid) -> usize {
    (p.0 + grid.columns as i32 * p.1) as usize
}

fn check_bounds(p: &Point, grid: &Grid) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.columns as i32 && p.1 < grid.rows as i32
}

fn parse_input(input: &str, grid: &Grid) -> (Vec<Point>, Vec<u32>) {
    let mut trailheads = vec![];
    let values = input
        .replace("\n", "")
        .chars()
        .enumerate()
        .filter_map(|(i, d)| {
            if let Some(d) = d.to_digit(10) {
                if d == 0 {
                    let p = index_to_position(i, grid);
                    trailheads.push(p);
                }
                return Some(d);
            }

            None
        })
        .collect::<Vec<u32>>();

    (trailheads, values)
}

fn climb(start: &Point, grid: &Grid, values: &Vec<u32>) -> Vec<Point> {
    let mut paths = vec![];

    let a = move_to(&start, &DIRS[0]);
    let b = move_to(&start, &DIRS[1]);
    let c = move_to(&start, &DIRS[2]);
    let d = move_to(&start, &DIRS[3]);

    traverse(&a, &0, &grid, &values, &mut paths);
    traverse(&b, &0, &grid, &values, &mut paths);
    traverse(&c, &0, &grid, &values, &mut paths);
    traverse(&d, &0, &grid, &values, &mut paths);

    paths
}

fn traverse(
    start: &Point,
    prev_value: &u32,
    grid: &Grid,
    values: &Vec<u32>,
    summits: &mut Vec<Point>,
) {
    if !check_bounds(start, grid) {
        return;
    }

    let idx = position_to_index(start, grid);
    let value = values[idx];

    if value != *prev_value + 1 {
        return;
    }

    if value == 9 && *prev_value == 8 {
        summits.push(Point(start.0, start.1));
        return;
    }

    let a = move_to(start, &DIRS[0]);
    let b = move_to(start, &DIRS[1]);
    let c = move_to(start, &DIRS[2]);
    let d = move_to(start, &DIRS[3]);

    traverse(&a, &value, grid, values, summits);
    traverse(&b, &value, grid, values, summits);
    traverse(&c, &value, grid, values, summits);
    traverse(&d, &value, grid, values, summits);
}

pub fn part_one(input: &str) -> Result<usize> {
    // trailheads start at elevation 0 and go to 9
    let mut lines = input.lines();
    let rows = input.lines().count();
    let columns = lines.next().expect("lol").len();
    let grid = Grid { rows, columns };

    let (trailheads, values) = parse_input(&input, &grid);
    let mut sum = 0;

    for th in &trailheads {
        let paths = climb(&th, &grid, &values);
        let summits: HashSet<Point> = HashSet::from_iter(paths);

        sum += summits.len()
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let rows = input.lines().count();
    let columns = lines.next().expect("lol").len();
    let grid = Grid { rows, columns };

    let (trailheads, values) = parse_input(&input, &grid);
    let mut sum = 0;

    for th in &trailheads {
        let paths = climb(&th, &grid, &values);
        sum += paths.len();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/010/example.txt") {
            assert_eq!(part_one(&input)?, 36);
        }

        if let Ok(input) = fs::read_to_string("./inputs/2024/010/input.txt") {
            println!("{}", part_one(&input)?);
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/010/example.txt") {
            assert_eq!(part_two(&input)?, 81);
        }

        if let Ok(input) = fs::read_to_string("./inputs/2024/010/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
