use std::collections::HashSet;

use anyhow::Result;

struct Grid {
    columns: i32,
    rows: i32,
}

fn index_to_position(idx: &i32, grid: &Grid) -> (i32, i32) {
    (idx % grid.columns, idx / grid.columns)
}

fn position_to_index(x: &i32, y: &i32, grid: &Grid) -> i32 {
    y * grid.columns + x
}

fn move_to(idx: &i32, x: &i32, y: &i32, grid: &Grid) -> i32 {
    let (cur_x, cur_y) = index_to_position(idx, grid);

    let new_x = cur_x + x;
    let new_y = cur_y + y;

    position_to_index(&new_x, &new_y, grid)
}

fn inside_bounds(idx: &i32, grid: &Grid) -> bool {
    let (x, y) = index_to_position(idx, grid);
    x >= 0 && y >= 0 && x < grid.columns && y < grid.rows
}

pub fn part_one(input: &str) -> Result<usize> {
    // Assming it always starts going up with `^`
    let chars = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut forward_dir = 0;

    let rows = input.lines().count() as i32;
    let columns = input.lines().map(|l| l.len()).max().unwrap() as i32;
    let grid = Grid { columns, rows };

    let mut visited: HashSet<i32> = HashSet::new();
    let mut guard = chars
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == '^' {
                return Some(i);
            }

            None
        })
        .next()
        .unwrap() as i32;

    while inside_bounds(&guard, &grid) {
        let (nx, ny) = dirs[forward_dir];
        let new_position = move_to(&guard, &nx, &ny, &grid);
        visited.insert(guard);

        match chars.get(new_position as usize) {
            Some(c) if *c == '#' => {
                forward_dir = (forward_dir + 1) % dirs.len();
                continue;
            }
            _ => {
                guard = new_position;
            }
        }
    }

    Ok(visited.len())
}

// pub fn part_two(input: &str) -> Result<i32> {
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/006/example.txt") {
            assert_eq!(part_one(&input)?, 41);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/006/input.txt") {
            println!("{}", part_one(&input)?);
        };

        Ok(())
    }

    // #[test]
    // fn two() -> Result<()> {
    //     if let Ok(input) = fs::read_to_string("./inputs/2024/005/example.txt") {
    //         assert_eq!(part_two(&input)?, 123);
    //     };
    //
    //     // 7730 too high
    //     if let Ok(input) = fs::read_to_string("./inputs/2024/005/input.txt") {
    //         println!("{}", part_two(&input)?);
    //     };
    //
    //     Ok(())
    // }
}
