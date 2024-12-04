use anyhow::Result;

// # Learnings
// - I can use slices of chars
// - Cancel out early when out of boundary or not matching the combination

struct Grid {
    columns: i32,
    rows: i32,
}

fn get_index(x: &i32, y: &i32, grid: &Grid) -> usize {
    let overflow_y = x / grid.columns;
    ((y + overflow_y) * grid.columns + x % grid.columns) as usize
}

fn get_position_from_index(idx: &i32, grid: &Grid) -> (i32, i32) {
    (idx % grid.columns, idx / grid.rows)
}

fn get_solutions(
    idx: &i32,
    input: &[char],
    grid: &Grid,
    dirs: &Vec<(i32, i32)>,
    combination: &Vec<char>,
) -> usize {
    let mut results = 0;
    let (sx, sy) = get_position_from_index(idx, grid);

    for dir in dirs.iter() {
        'inner: for j in 0..4 {
            let x = sx + dir.0 * j as i32;
            let y = sy + dir.1 * j as i32;

            if x >= grid.columns || y >= grid.rows || x < 0 || y < 0 {
                break 'inner;
            }

            let new_index = get_index(&x, &y, grid);

            if input[new_index] != combination[j] {
                break 'inner;
            }

            if j == 3 {
                results += 1;
            }
        }
    }

    results
}

fn get_solutions_cross(idx: &i32, input: &[char], grid: &Grid) -> usize {
    let pattern = vec!['M', 'S']; // this is already sorted
    let left = vec![(-1, -1), (1, 1)];
    let right = vec![(1, -1), (-1, 1)];

    let (sx, sy) = get_position_from_index(idx, grid);

    let mut l = left
        .iter()
        .map(|dir| {
            let x = sx + dir.0 as i32;
            let y = sy + dir.1 as i32;

            if x >= grid.columns || y >= grid.rows || x < 0 || y < 0 {
                return '.';
            }

            let new_index = get_index(&x, &y, grid);

            input[new_index]
        })
        .collect::<Vec<char>>();

    let mut r = right
        .iter()
        .map(|dir| {
            let x = sx + dir.0 as i32;
            let y = sy + dir.1 as i32;

            if x >= grid.columns || y >= grid.rows || x < 0 || y < 0 {
                return '.';
            }

            let new_index = get_index(&x, &y, grid);

            input[new_index]
        })
        .collect::<Vec<char>>();

    l.sort();
    r.sort();

    match l == pattern && r == pattern {
        true => 1,
        false => 0,
    }
}

pub fn part_one(input: &str) -> Result<i32> {
    let combination = vec!['X', 'M', 'A', 'S'];
    let dirs = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
    ];
    let lines = input.lines().collect::<Vec<&str>>();
    let rows = lines.len() as i32;
    let columns = lines.first().unwrap().len() as i32;
    let grid = Grid { rows, columns };

    let input_chars = input.chars().filter(|c| c != &'\n').collect::<Vec<char>>();

    let sum = input_chars
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            'X' => get_solutions(&(i as i32), &input_chars, &grid, &dirs, &combination) as i32,
            _ => 0,
        })
        .sum();

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<i32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let rows = lines.len() as i32;
    let columns = lines.first().unwrap().len() as i32;
    let grid = Grid { rows, columns };

    let input_chars = input.chars().filter(|c| c != &'\n').collect::<Vec<char>>();

    let sum = input_chars
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            'A' => get_solutions_cross(&(i as i32), &input_chars, &grid) as i32,
            _ => 0,
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
        let example = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_one(example)?, 18);

        // 2529 -- Too High
        // 2512 -- Too High
        if let Ok(input) = fs::read_to_string("./inputs/2024/004/input.txt") {
            println!("Day 01 | Part 01: {}", part_one(&input)?)
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        let example = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!(part_two(example)?, 9);

        if let Ok(input) = fs::read_to_string("./inputs/2024/004/input.txt") {
            println!("Day 01 | Part 02: {}", part_two(&input)?)
        }

        Ok(())
    }
}
