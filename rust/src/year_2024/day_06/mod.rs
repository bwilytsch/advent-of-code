use std::collections::HashSet;

use anyhow::Result;

struct Grid {
    columns: i32,
    rows: i32,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct PathNode {
    coord: i32,
    dir: usize,
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

fn is_inbounds(p0: &Point, grid: &Grid) -> bool {
    p0.x >= 0 && p0.y >= 0 && p0.x < grid.columns && p0.y < grid.rows
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

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn from_index(idx: &i32, grid: &Grid) -> Self {
        let (x, y) = index_to_position(idx, grid);
        Point::new(x, y)
    }

    fn add_point(&mut self, p0: Point) -> Self {
        Point::new(self.x + p0.x, self.y + p0.y)
    }
}

// NOTE: This is only unique with a hashset, I need to be able to properly dedpulicate my points
fn explore(start: &i32, start_dir: &usize, grid: &Grid, chars: &Vec<char>) -> Vec<(Point, usize)> {
    let mut adjusted_chars = chars.clone();
    let mut unique: HashSet<Point> = HashSet::new();
    let mut visited: Vec<(Point, usize)> = Vec::new();
    let mut dir = start_dir.clone();
    let dirs = vec![
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    let mut pos = Point::from_index(start, grid);

    while is_inbounds(&pos, &grid) {
        if !unique.contains(&pos) {
            visited.push((pos, dir));
            unique.insert(pos);
        }

        let current_idx = position_to_index(&pos.x, &pos.y, grid);
        adjusted_chars[current_idx as usize] = 'X';

        let heading = dirs[dir];
        let new_position = pos.add_point(heading);
        let next = position_to_index(&new_position.x, &new_position.y, grid);

        match adjusted_chars.get(next as usize) {
            Some(c) if *c == '#' => {
                dir = (dir + 1) % dirs.len();
                continue;
            }
            _ => {
                pos = new_position;
            }
        }
    }

    visited
}

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn check_loop_fast(start: &(Point, usize), grid: &Grid, chars: &Vec<char>) -> bool {
    let mut dir = start.1;
    let mut pos = start.0.clone();

    // Pre-calculate grid bounds for faster checks
    let max_x = grid.columns - 1;
    let max_y = grid.rows - 1;

    // Use a more efficient data structure for position+direction tracking
    // Assuming your grid isn't huge, we can use a flat array/vec instead of HashSet
    let mut visited = vec![false; (grid.columns as usize) * (grid.rows as usize) * 4];

    loop {
        // Fast bounds check with pre-calculated values
        if pos.x < 0 || pos.y < 0 || pos.x > max_x || pos.y > max_y {
            break;
        }

        // Convert current state to single index for visited array
        let visit_index = (pos.x + pos.y * grid.columns) as usize * 4 + dir;

        if visited[visit_index] {
            return true;
        }
        visited[visit_index] = true;

        // Use constant array instead of vector
        let (dx, dy) = DIRS[dir];
        let new_x = pos.x + dx;
        let new_y = pos.y + dy;

        // Early bounds check before creating new Point
        if new_x < 0 || new_y < 0 || new_x > max_x || new_y > max_y {
            break;
        }

        let next = position_to_index(&new_x, &new_y, grid);

        match chars.get(next as usize) {
            Some(&'O' | &'#') => {
                dir = (dir + 1) & 3; // Fast modulo for power of 2
            }
            _ => {
                pos.x = new_x;
                pos.y = new_y;
            }
        }
    }
    false
}

pub fn part_two(input: &str) -> Result<usize> {
    // Assming it always starts going up with `^`
    let chars = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let forward_dir = 0;

    let rows = input.lines().count() as i32;
    let columns = input.lines().map(|l| l.len()).max().unwrap() as i32;
    let grid = Grid { columns, rows };

    let guard = chars
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

    let solved_map = explore(&guard, &forward_dir, &grid, &chars);

    let mut count = 0;
    let start = solved_map.first().unwrap();

    for v in solved_map.iter() {
        let mut adjusted_chars = chars.clone();
        let obstacle = position_to_index(&v.0.x, &v.0.y, &grid) as usize;
        adjusted_chars[obstacle] = 'O';

        if check_loop_fast(&start, &grid, &adjusted_chars) {
            count += 1;
        }
    }

    Ok(count)
}

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

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/006/example.txt") {
            assert_eq!(part_two(&input)?, 6);
        };

        if let Ok(input) = fs::read_to_string("./inputs/2024/006/input.txt") {
            println!("{}", part_two(&input)?);
        }

        Ok(())
    }
}
