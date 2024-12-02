use std::cmp::PartialEq;
#[derive(Debug, Clone)]
enum Direction {
    Decreasing,
    Increasing,
}
impl Direction {
    pub fn check_direction(a: i64, b: i64) -> Self {
        if (a < b) {
            return Direction::Increasing;
        }

        Direction::Decreasing
    }
    pub fn check_if_safe(v: &Vec<i64>, dir: Self) -> bool {
        let min: i64 = 1;
        let max: i64 = 3;
        for (i, y) in v.iter().enumerate() {
            if (i != v.len() - 1) {
                let x = match dir {
                    Direction::Decreasing => y - v[i + 1],
                    Direction::Increasing => v[i + 1] - y,
                };
                if !(x >= min && x <= max) {
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Decreasing, Direction::Decreasing) => true,
            (Direction::Increasing, Direction::Increasing) => true,
            _ => false,
        }
    }
}

fn p_1() {
    let lines: Vec<Vec<i64>> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let safe = lines
        .iter()
        .map(|line| {
            let dir = Direction::check_direction(line[0], line[1]);
            if Direction::check_if_safe(line, dir) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<i32>();

    println!("part 1 : {:?}", safe);
}

fn p_2() {
    let mut lines: Vec<Vec<i64>> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let safe = lines
        .iter_mut()
        .map(|mut line| {
            let dir = Direction::check_direction(line[0], line[1]);
            if Direction::check_if_safe(line, dir.clone()) {
                return 1;
            } else {
                let mut res = 0;
                for (index, _) in line.iter().enumerate() {
                    let mut new_line = line.clone();
                    new_line.remove(index);
                    let new_dir = Direction::check_direction(new_line[0], new_line[1]);
                    if (Direction::check_if_safe(&new_line, new_dir)) {
                        res = 1;
                        break;
                    }
                }
                res
            }
        })
        .sum::<i32>();

    println!("part 2 : {:?}", safe);
}

fn main() {
    p_1();
    p_2();
}
