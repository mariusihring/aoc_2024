use std::io::{self, BufRead};
use std::str::Lines;

const WORD_1: &str = "XMAS";
const WORD_2: &str = "MAS";

fn p_1(input: Lines) {
    let grid: Vec<Vec<char>> = input.map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            count += count_occurrences(&grid, r, c, rows, cols, WORD_1);
        }
    }

    println!("P1: {}", count);
}

fn p_2(input: Lines) {
    let grid: Vec<Vec<char>> = input.map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if (grid[r][c] == 'A') {
                count += check_mas_x_shapes(&grid, r, c, rows, cols);
            }
        }
    }

    println!("P2: {}", count);
}

fn main() {
    let input = std::fs::read_to_string("pascal").unwrap();
    let input = input.lines();

    p_1(input.clone());
    p_2(input.clone());
}

fn count_occurrences(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
    word: &str,
) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Up-left
        (-1, 1),  // Up-right
    ];
    let mut count = 0;
    for (dr, dc) in directions.iter() {
        if let Some(occurrence) = check_word(grid, r, c, rows, cols, *dr, *dc, word) {
            count += occurrence;
        }
    }
    count
}

fn check_mas_x_shapes(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> usize {
    if r >= 1 && r + 1 < rows && c >= 1 && c + 1 < cols {
        let mut left_right = String::new();
        let mut right_left = String::new();
        left_right.push(grid[r - 1][c - 1]);
        left_right.push(grid[r][c]);
        left_right.push(grid[r + 1][c + 1]);

        right_left.push(grid[r - 1][c + 1]);
        right_left.push(grid[r][c]);
        right_left.push(grid[r + 1][c - 1]);

        if ((left_right.contains(WORD_2)
            || left_right
                .chars()
                .rev()
                .collect::<String>()
                .contains(WORD_2))
            && (right_left
                .chars()
                .rev()
                .collect::<String>()
                .contains(WORD_2)
                || right_left.contains(WORD_2)))
        {
            return 1;
        }
    }

    0
}

fn check_word(
    grid: &Vec<Vec<char>>,
    mut r: usize,
    mut c: usize,
    rows: usize,
    cols: usize,
    dr: i32,
    dc: i32,
    word: &str,
) -> Option<usize> {
    let chars: Vec<char> = word.chars().collect();
    for &ch in chars.iter() {
        if r >= rows || c >= cols || grid[r][c] != ch {
            return None;
        }
        r = (r as isize + dr as isize) as usize;
        c = (c as isize + dc as isize) as usize;
    }
    Some(1)
}
