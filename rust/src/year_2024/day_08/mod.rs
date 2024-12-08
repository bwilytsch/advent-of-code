use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
};

use anyhow::Result;

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x: {}, y:{} ", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> Point {
        Point::new(self.x - other.x as i32, self.y - other.y as i32)
    }

    fn add(&self, p: &Point) -> Self {
        Point::new(self.x + p.x, self.y + p.y)
    }

    fn sub(&self, p: &Point) -> Self {
        Point::new(self.x - p.x, self.y - p.y)
    }
}

struct Grid {
    rows: usize,
    columns: usize,
}

fn in_bounds(p: &Point, grid: &Grid) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < (grid.columns as i32) && p.y < (grid.rows as i32)
}

pub fn part_one(input: &str) -> Result<usize> {
    let rows = input.lines().collect::<Vec<_>>();
    let columns = rows[0].len();
    let grid = Grid {
        rows: rows.len(),
        columns,
    };

    let mut lot: HashMap<char, Vec<Point>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let entry = lot.entry(c).or_insert(vec![]);
                entry.push(Point {
                    x: column as i32,
                    y: row as i32,
                })
            }
        }
    }

    let mut unique_antinodes: HashSet<Point> = HashSet::new();

    for (_, points) in lot {
        for p0 in points.iter() {
            for j in 0..points.len() {
                let p1 = points[j];

                let d = p0.distance(&p1);

                let values = vec![p0.sub(&d), p1.add(&d), p0.add(&d), p1.sub(&d)];

                values.iter().filter(|p| in_bounds(p, &grid)).for_each(|p| {
                    if p != p0 && p != &p1 {
                        unique_antinodes.insert(p.clone());
                    }
                });
            }
        }
    }

    Ok(unique_antinodes.iter().count())
}

pub fn part_two(input: &str) -> Result<usize> {
    let rows = input.lines().collect::<Vec<_>>();
    let columns = rows[0].len();
    let grid = Grid {
        rows: rows.len(),
        columns,
    };

    let mut lot: HashMap<char, Vec<Point>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let entry = lot.entry(c).or_insert(vec![]);
                entry.push(Point {
                    x: column as i32,
                    y: row as i32,
                })
            }
        }
    }

    let mut unique_antinodes: HashSet<Point> = HashSet::new();

    for (_, points) in lot {
        for p0 in points.iter() {
            for j in 0..points.len() {
                let p1 = points[j];

                if p0 == &p1 {
                    continue;
                }

                let d = p0.distance(&p1);

                let mut cur = p0.clone();
                while in_bounds(&cur, &grid) {
                    unique_antinodes.insert(cur.clone());
                    cur = cur.add(&d);
                }

                let mut cur = p0.clone();
                while in_bounds(&cur, &grid) {
                    unique_antinodes.insert(cur.clone());
                    cur = cur.sub(&d);
                }
            }
        }
    }

    Ok(unique_antinodes.iter().count())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/008/example.txt") {
            assert_eq!(part_one(&input)?, 14);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/008/input.txt") {
            println!("{}", part_one(&input)?);
        };

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/008/example.txt") {
            assert_eq!(part_two(&input)?, 34);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/008/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
