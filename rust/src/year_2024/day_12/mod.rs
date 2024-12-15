use std::{collections::HashSet, hash::Hash};

use anyhow::Result;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

// These a structured as follows First, Opposite, Second
const CORNERS: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (-1, 0), (0, -1)],
    [(1, -1), (1, 0), (0, -1)],
    [(1, 1), (1, 0), (0, 1)],
    [(-1, 1), (-1, 0), (0, 1)],
];

#[derive(Hash, Clone, Debug)]
struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

struct Grid {
    rows: usize,
    columns: usize,
}

fn flood(
    start: &Point,
    term: &char,
    map: &Vec<Vec<char>>,
    grid: &Grid,
    seen: &mut HashSet<Point>,
    area: &mut HashSet<Point>,
) {
    if seen.contains(start) {
        return;
    }

    if map[start.1 as usize][start.0 as usize] != *term {
        return;
    }

    seen.insert(start.clone());
    area.insert(start.clone());

    for p in DIRS.iter().map(|d| Point(start.0 + d.0, start.1 + d.1)) {
        if p.0 < 0 || p.1 < 0 || p.0 >= (grid.columns as i32) || p.1 >= (grid.rows as i32) {
            continue;
        };

        flood(&p, term, map, grid, seen, area);
    }
}

fn get_area(start: &Point, map: &Vec<Vec<char>>, grid: &Grid) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut area = HashSet::from([start.clone()]);
    let term = map[start.1 as usize][start.0 as usize];

    flood(start, &term, map, grid, &mut visited, &mut area);

    area
}

fn get_perimeter(area: &HashSet<Point>) -> usize {
    let mut output = 0;

    for ap in area {
        output += 4;

        for p in DIRS.iter().map(|d| Point(ap.0 + d.0, ap.1 + d.1)) {
            if area.contains(&p) {
                output -= 1;
            }
        }
    }

    output
}

fn get_plot(p: &Point, map: &Vec<Vec<char>>) -> char {
    let x = p.0;
    let y = p.1;
    let rows = map.len() as i32;
    let columns = map[0].len() as i32;

    if x < 0 || y < 0 || x >= columns || y >= rows {
        return '.';
    }

    map[y as usize][x as usize]
}

// Calculating half-points
fn get_corners(area: &HashSet<Point>, map: &Vec<Vec<char>>) -> usize {
    let mut corners = 0;

    for p in area {
        let Point(px, py) = p;
        let pt = get_plot(p, map);

        for [oc, fc, sc] in CORNERS {
            let o = get_plot(&Point(px + oc.0, py + oc.1), map);
            let f = get_plot(&Point(px + fc.0, py + fc.1), map);
            let s = get_plot(&Point(px + sc.0, py + sc.1), map);

            if (f != pt && s != pt) || (f == pt && s == pt && o != pt) {
                corners += 1;
            }
        }
    }

    corners
}

pub fn part_one(input: &str) -> Result<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    let rows = lines.len();
    let columns = lines[0].len();

    let grid = Grid { rows, columns };

    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut areas = vec![];
    let mut seen = HashSet::new();

    for r in 0..rows {
        for c in 0..columns {
            let start = Point(c as i32, r as i32);

            if seen.contains(&start) {
                continue;
            }

            seen.insert(start.clone());

            let area = get_area(&start, &map, &grid);

            seen.extend(area.clone());
            areas.push(area);
        }
    }

    let sum: usize = areas
        .iter()
        .map(|a| {
            let count = get_perimeter(&a);

            return count * a.len();
        })
        .sum();

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    let rows = lines.len();
    let columns = lines[0].len();

    let grid = Grid { rows, columns };

    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut areas = vec![];
    let mut seen = HashSet::new();

    for r in 0..rows {
        for c in 0..columns {
            let start = Point(c as i32, r as i32);

            if seen.contains(&start) {
                continue;
            }

            seen.insert(start.clone());

            let area = get_area(&start, &map, &grid);

            seen.extend(area.clone());
            areas.push(area);
        }
    }

    let sum: usize = areas
        .iter()
        .map(|a| {
            let corners = get_corners(&a, &map);

            return corners * a.len();

            // return count * a.len();
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        assert_eq!(
            part_one(
                "OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO"
            )?,
            772
        );

        assert_eq!(
            part_one(
                "AAAA
        BBCD
        BBCC
        EEEC"
            )?,
            140
        );

        if let Ok(input) = fs::read_to_string("./inputs/2024/012/example.txt") {
            assert_eq!(part_one(&input)?, 1930);
        }

        if let Ok(input) = fs::read_to_string("./inputs/2024/012/input.txt") {
            println!("{}", part_one(&input)?);
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        // AAAA
        // BBCD
        // BBCC
        // EEEC
        assert_eq!(
            part_two(
                "AAAA
        BBCD
        BBCC
        EEEC"
            )?,
            80
        );

        assert_eq!(
            part_two(
                "EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE"
            )?,
            236
        );

        assert_eq!(
            part_two(
                "AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA"
            )?,
            368
        );

        if let Ok(input) = fs::read_to_string("./inputs/2024/012/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
