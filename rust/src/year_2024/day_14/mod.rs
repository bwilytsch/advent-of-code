use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone)]
struct Vector(isize, isize);

#[derive(Debug)]
pub struct Room {
    min: Vector,
    max: Vector,
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Vector,
    v: Vector,
}

impl Robot {
    fn move_on_grid(&mut self, t: isize, sector: &Room) {
        fn wrap(pos: isize, velocity: isize, t: isize, max: isize) -> isize {
            let new_pos = pos + velocity * t;
            ((new_pos % max) + max) % max
        }

        self.pos.0 = wrap(self.pos.0, self.v.0, t, sector.max.0);
        self.pos.1 = wrap(self.pos.1, self.v.1, t, sector.max.1);
    }
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"p\=(\d+),(\d+) v\=(-?\d+),(-?\d+)").unwrap();

        if let Some(caps) = re.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let vx = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let vy = caps.get(4).unwrap().as_str().parse::<isize>().unwrap();

            return Robot {
                pos: Vector(x, y),
                v: Vector(vx, vy),
            };
        }

        panic!("This should not happen");
    }
}

fn print_robots(robots: &Vec<Robot>, room: &Room) {}

pub fn part_one(input: &str, room: Room, t: isize) -> Result<isize> {
    let robots = input.lines().map(|l| l.into()).collect::<Vec<Robot>>();
    let w = room.max.0;
    let h = room.max.1;
    let hw = w / 2; // Half-width (integer division)
    let hh = h / 2; // Half-height (integer division)

    // Define quadrants while skipping the center row and column
    let mut quadrants = [
        (
            Room {
                min: Vector(0, 0),
                max: Vector(hw, hh),
            },
            vec![],
        ), // Top-left
        (
            Room {
                min: Vector(hw + 1, 0),
                max: Vector(w, hh),
            },
            vec![],
        ), // Top-right
        (
            Room {
                min: Vector(0, hh + 1),
                max: Vector(hw, h),
            },
            vec![],
        ), // Bottom-left
        (
            Room {
                min: Vector(hw + 1, hh + 1),
                max: Vector(w, h),
            },
            vec![],
        ), // Bottom-right
    ];

    for mut r in robots {
        r.move_on_grid(t, &room);
        let x = r.pos.0;
        let y = r.pos.1;

        for q in quadrants.iter_mut() {
            if x >= q.0.min.0 && y >= q.0.min.1 && x < q.0.max.0 && y < q.0.max.1 {
                q.1.push(r);
                break;
            }
        }
    }

    let sum: isize = quadrants
        .iter()
        .fold(1, |acc, (_, robots)| acc * (robots.len() as isize));

    Ok(sum)
}

pub fn part_two(input: &str, room: Room, t: isize) -> Result<isize> {
    let robots = input.lines().map(|l| l.into()).collect::<Vec<Robot>>();

    for mut r in robots.clone() {
        r.move_on_grid(t, &room);
        let x = r.pos.0;
        let y = r.pos.1;
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn one() -> Result<()> {
        let example = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!(
            part_one(
                example,
                Room {
                    min: Vector(0, 0),
                    max: Vector(11, 7),
                },
                100
            )?,
            12,
        );

        if let Ok(input) = fs::read_to_string("./inputs/2024/014/input.txt") {
            println!(
                "{}",
                part_one(
                    &input,
                    Room {
                        min: Vector(0, 0),
                        max: Vector(101, 103),
                    },
                    100
                )?
            )
        }

        Ok(())
    }

    #[test]
    fn two() -> Result<()> {
        if let Ok(input) = fs::read_to_string("./inputs/2024/014/input.txt") {
            println!(
                "{}",
                part_two(
                    &input,
                    Room {
                        min: Vector(0, 0),
                        max: Vector(101, 103),
                    },
                    0
                )?
            )
        }
        Ok(())
    }
}
