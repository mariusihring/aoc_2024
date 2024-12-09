use glam::IVec2;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    p_1(&input);
    p_2(&input);
}

pub fn p_1(input: &str) {
    println!("P1: {}", solve_puzzle(input));
}

pub fn p_2(input: &str) {
    println!("P2: {}", solve_puzzle_harmonics(input))
}

#[derive(Debug)]
struct Coverage {
    matrix: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
    antinodes: HashMap<IVec2, char>,
}

#[derive(Debug)]
struct Tower {
    value: char,
    pos: IVec2,
}

impl Coverage {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        let max_x = (matrix.len() - 1) as i32;
        assert!(max_x >= 0, "Matrix max_x must be >= 0");
        let max_y = (matrix[0].len() - 1) as i32;
        assert!(max_y >= 0, "Matrix max_y must be >= 0");

        Self {
            matrix,
            max_x,
            max_y,
            antinodes: HashMap::new(),
        }
    }

    fn find_towers(&self) -> Vec<Tower> {
        let mut towers: Vec<Tower> = Vec::new();
        for (x, xv) in self.matrix.iter().enumerate() {
            for (y, yv) in xv.iter().enumerate() {
                if yv.is_alphanumeric() {
                    let pos = IVec2::new(x as i32, y as i32);
                    towers.push(Tower { value: *yv, pos });
                }
            }
        }
        towers
    }

    fn find_item(&self, x: i32, y: i32) -> Option<&char> {
        if x >= 0 && y >= 0 && x <= self.max_x && y <= self.max_y {
            return Some(&self.matrix[x as usize][y as usize]);
        }
        None
    }

    fn scan_section(&self, tower: &Tower) -> HashSet<(IVec2, IVec2)> {
        let mut pairs: HashSet<(IVec2, IVec2)> = HashSet::new();
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let px = tower.pos.x + x;
                let py = tower.pos.y + y;
                if px == tower.pos.x && py == tower.pos.y {
                    continue;
                }

                let mut right_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let fpos = IVec2::new(px, py);
                        if is_coord_greater(&fpos, &tower.pos) {
                            pairs.insert((tower.pos, fpos));
                        } else {
                            pairs.insert((fpos, tower.pos));
                        }
                    }
                } else {
                    right_none = true;
                }

                let px = tower.pos.x + x;
                let py = tower.pos.y - y;
                let mut left_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let bpos = IVec2::new(px, py);
                        if is_coord_greater(&bpos, &tower.pos) {
                            pairs.insert((tower.pos, bpos));
                        } else {
                            pairs.insert((bpos, tower.pos));
                        }
                    }
                } else {
                    left_none = true;
                }

                if right_none && left_none {
                    break;
                }
            }
        }

        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let px = tower.pos.x - x;
                let py = tower.pos.y + y;
                if px == tower.pos.x && py == tower.pos.y {
                    continue;
                }

                let mut right_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let fpos = IVec2::new(px, py);
                        if is_coord_greater(&fpos, &tower.pos) {
                            pairs.insert((tower.pos, fpos));
                        } else {
                            pairs.insert((fpos, tower.pos));
                        }
                    }
                } else {
                    right_none = true;
                }

                let px = tower.pos.x - x;
                let py = tower.pos.y - y;
                let mut left_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let bpos = IVec2::new(px, py);
                        if is_coord_greater(&bpos, &tower.pos) {
                            pairs.insert((tower.pos, bpos));
                        } else {
                            pairs.insert((bpos, tower.pos));
                        }
                    }
                } else {
                    left_none = true;
                }

                if right_none && left_none {
                    break;
                }
            }
        }

        pairs
    }

    fn plot_antinodes(&mut self, tower: &Tower, pairs: &HashSet<(IVec2, IVec2)>) {
        for pair in pairs.iter() {
            let prev = get_prev_coord(&pair.0, &pair.1);
            if self.find_item(prev.x, prev.y).is_some() {
                self.antinodes.insert(prev, tower.value);
            }
            let next = get_next_coord(&pair.0, &pair.1);
            if self.find_item(next.x, next.y).is_some() {
                self.antinodes.insert(next, tower.value);
            }
        }
    }

    fn plot_harmonics_antinodes(&mut self, tower: &Tower, pairs: &HashSet<(IVec2, IVec2)>) {
        for pair in pairs.iter() {
            self.antinodes.insert(pair.0, tower.value);
            self.antinodes.insert(pair.1, tower.value);

            let mut a: IVec2 = pair.0;
            let mut b: IVec2 = pair.1;
            loop {
                let prev = get_prev_coord(&a, &b);
                if self.find_item(prev.x, prev.y).is_some() {
                    self.antinodes.insert(prev, tower.value);
                    b = a;
                    a = prev;
                } else {
                    break;
                }
            }

            let mut a: IVec2 = pair.0;
            let mut b: IVec2 = pair.1;
            loop {
                let next = get_next_coord(&a, &b);
                if self.find_item(next.x, next.y).is_some() {
                    self.antinodes.insert(next, tower.value);
                    a = b;
                    b = next;
                } else {
                    break;
                }
            }
        }
    }
}

fn solve_puzzle(input: &str) -> i32 {
    let mut coverage = parse_data(input);
    let towers = coverage.find_towers();

    for tower in towers.iter() {
        let pairs = coverage.scan_section(tower);
        coverage.plot_antinodes(tower, &pairs);
    }
    coverage.antinodes.len() as i32
}

fn solve_puzzle_harmonics(input: &str) -> i32 {
    let mut coverage = parse_data(input);
    let towers = coverage.find_towers();

    for tower in towers.iter() {
        let pairs = coverage.scan_section(tower);
        coverage.plot_harmonics_antinodes(tower, &pairs);
    }
    coverage.antinodes.len() as i32
}

fn parse_data(data: &str) -> Coverage {
    let matrix: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    Coverage::new(matrix)
}

fn is_coord_greater(v1: &IVec2, v2: &IVec2) -> bool {
    if v1.x > v2.x {
        return true;
    }
    if v1.x == v2.x {
        return v1.y > v2.y;
    }
    false
}

fn get_next_coord(a: &IVec2, b: &IVec2) -> IVec2 {
    let diff_x = b.x - a.x;
    let diff_y = b.y - a.y;
    IVec2::new(b.x + diff_x, b.y + diff_y)
}

fn get_prev_coord(a: &IVec2, b: &IVec2) -> IVec2 {
    let diff_x = b.x - a.x;
    let diff_y = b.y - a.y;
    IVec2::new(a.x - diff_x, a.y - diff_y)
}
