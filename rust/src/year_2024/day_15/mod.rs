use std::fmt::Display;

use anyhow::Result;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("This should not happen"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
        }
    }
}

fn walk(p: &(usize, usize), dir: &Direction) -> (usize, usize) {
    let mut next = p.clone();

    match dir {
        Direction::Up => {
            next.1 -= 1;
        }
        Direction::Left => {
            next.0 -= 1;
        }
        Direction::Down => {
            next.1 += 1;
        }
        Direction::Right => {
            next.0 += 1;
        }
    }

    next
}

fn push(
    p: &mut (usize, usize),
    dir: &Direction,
    o: &mut Vec<(usize, usize)>,
    w: &Vec<(usize, usize)>,
) {
    println!("In: {:?}", p);

    let n = walk(p, dir);

    if w.contains(&n) {
        println!("hit a wall at {:?}", p);
        return;
    }

    let nn = walk(&n, dir);

    // Move foward while not a still space until a wall
    if let Some(oo) = o.iter_mut().find(|(ox, oy)| *ox == p.0 && *oy == p.1) {
        if !w.contains(&nn) {
            println!("push obstacle to {:?}", nn);
            oo.0 = nn.0;
            oo.1 = nn.1;
        }
    }

    p.0 = n.0;
    p.1 = n.1;

    println!("Out: {:?}", p);
}

pub fn part_one(input: &str) -> Result<usize> {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();

    let mut robot = (0, 0);
    let mut goods = vec![];
    let mut walls = vec![];

    for (i, l) in warehouse.lines().enumerate() {
        for (j, c) in l.trim().chars().enumerate() {
            if c == '@' {
                robot = (j, i)
            }

            if c == 'O' {
                goods.push((j, i))
            }

            if c == '#' {
                walls.push((j, i))
            }
        }
    }

    let directions = moves
        .trim()
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>();

    for d in directions {
        push(&mut robot, &d, &mut goods, &walls);
    }

    println!("robot: {:?}, goods: {:?}", robot, goods);

    Ok(0)
}

// fn part_two(_input: &str) -> Result<usize> {
//     Ok(0)
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        if let Ok(example) = fs::read_to_string("./inputs/2024/015/example.txt") {
            assert_eq!(part_one(&example)?, 2028);
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        Ok(())
    }
}
