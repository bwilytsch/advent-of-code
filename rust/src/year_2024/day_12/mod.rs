use std::collections::HashSet;

use anyhow::Result;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn move_to(index: &usize, dir: &(i32, i32), rows: &usize, columns: &usize) -> Option<usize> {
    let c = *columns as i32;
    let r = *rows as i32;
    let x = (index % columns) as i32;
    let y = (index / columns) as i32;

    // Move
    let nx = x + dir.0;
    let ny = y + dir.1;

    if nx >= 0 && ny >= 0 && nx < c && ny < r {
        return Some((nx + c * ny) as usize);
    }

    None
}

fn collect(
    start: &usize,
    term: &char,
    rows: &usize,
    columns: &usize,
    map: &Vec<char>,
    visited: &mut HashSet<usize>,
    area: &mut HashSet<usize>,
) {
    let c = map[*start];

    if *term != c {
        return;
    }

    if visited.contains(start) {
        return;
    }

    visited.insert(*start);
    area.insert(*start);
    println!("{:?} -> {:?}", visited, area);

    for dir in &DIRS {
        if let Some(next) = move_to(start, dir, &rows, &columns) {
            collect(&next, term, rows, columns, map, visited, area);
        }
    }
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let rows = input.lines().count();
    let columns = lines.next().expect("lol").len();

    let mut visited = HashSet::new();
    let map = input.chars().filter(|c| *c != '\n').collect::<Vec<_>>();

    let areas = map
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let mut area = HashSet::new();
            collect(&i, &c, &rows, &columns, &map, &mut visited, &mut area);

            if area.len() > 0 {
                return Some(area);
            }

            None
        })
        .collect::<Vec<_>>();

    println!("{:?}", areas);

    Ok(areas.len())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        assert_eq!(
            part_one(
                "AAAA
BBCD
BBCC
EEEC
"
            )?,
            5
        );

        if let Ok(input) = fs::read_to_string("./inputs/2024/011/example.txt") {
            assert_eq!(part_one(&input)?, 1930);
        }

        Ok(())
    }
}
