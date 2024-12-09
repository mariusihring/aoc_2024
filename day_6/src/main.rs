use std::collections::HashSet;

fn get_grid() -> Vec<Vec<char>> {
    let input = std::fs::read_to_string("input").unwrap();
    input.lines().map(|line| line.chars().collect()).collect()
}
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

#[derive(Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other),
            (Direction::Up, Direction::Up) |
            (Direction::Right, Direction::Right) |
            (Direction::Down, Direction::Down) |
            (Direction::Left, Direction::Left))
    }
}

fn simulate_guard_path(grid: &Vec<Vec<char>>, start_pos: (usize, usize), start_direction: Direction) -> HashSet<(usize, usize)> {
    let directions = vec![
        (Direction::Up, (-1, 0)),
        (Direction::Right, (0, 1)),
        (Direction::Down, (1, 0)),
        (Direction::Left, (0, -1)),
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut x = start_pos.0;
    let mut y = start_pos.1;
    let mut direction = start_direction;
    let mut visited = HashSet::new();
    visited.insert((x, y));
    let mut steps = 0;
    let max_steps = rows * cols * 4;

    while x < rows && y < cols && steps < max_steps {
        let (dx, dy) = directions.iter().find(|(dir, _)| *dir == direction).unwrap().1;
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if nx < rows && ny < cols && grid[nx][ny] != '#' {
            x = nx;
            y = ny;
        } else {
            // Turn right
            direction = direction.turn_right();
        }

        if !visited.insert((x, y)) {
            break;
        }
        steps += 1;
    }

    visited
}

fn detect_loop(grid: &Vec<Vec<char>>, start_pos: (usize, usize), start_direction: Direction) -> bool {
    let directions = vec![
        (Direction::Up, (-1, 0)),
        (Direction::Right, (0, 1)),
        (Direction::Down, (1, 0)),
        (Direction::Left, (0, -1)),
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited_states = HashSet::new();
    let mut current_state = State {
        pos: start_pos,
        dir: start_direction,
    };

    while current_state.pos.0 < rows && current_state.pos.1 < cols {
        if !visited_states.insert(current_state) {
            return true;
        }

        let (dx, dy) = directions.iter()
            .find(|(dir, _)| *dir == current_state.dir)
            .unwrap()
            .1;

        let nx = (current_state.pos.0 as isize + dx) as usize;
        let ny = (current_state.pos.1 as isize + dy) as usize;

        if nx < rows && ny < cols && grid[nx][ny] != '#' {
            current_state.pos = (nx, ny);
        } else {
            current_state.dir = current_state.dir.turn_right();
        }

        if nx >= rows || ny >= cols {
            return false;
        }
    }

    false
}



fn p_1() {
    let grid = get_grid();
    let mut visited = simulate_guard_path(&grid, (0, 0), Direction::Up);

    // Find the starting position and initial direction
    let mut position = (0, 0);
    let mut curr_direction = Direction::Up;
    for (index, line) in grid.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| c == &'^' || c == &'>' || c == &'<' || c == &'v') {
            position = (index, x);
            curr_direction = match grid[position.0][position.1] {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => unreachable!(),
            };
            break;
        }
    }

    visited = simulate_guard_path(&grid, position, curr_direction);

    println!("Visited cells: {}", visited.len());
}


fn p_2() {
    let mut valid_pos: HashSet<(usize, usize)> = HashSet::new();
    let original_grid = get_grid();

    let mut start_pos = (0, 0);
    let mut start_dir = Direction::Up;
    for (index, line) in original_grid.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| *c == '^' || *c == '>' || *c == '<' || *c == 'v') {
            start_pos = (index, x);
            start_dir = match original_grid[start_pos.0][start_pos.1] {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => unreachable!(),
            };
            break;
        }
    }

    for row_index in 0..original_grid.len() {
        for col_index in 0..original_grid[0].len() {
            if original_grid[row_index][col_index] == '.' && (row_index, col_index) != start_pos {
                let mut test_grid = original_grid.clone();
                test_grid[row_index][col_index] = '#';

                if detect_loop(&test_grid, start_pos, start_dir) {
                    valid_pos.insert((row_index, col_index));
                }
            }
        }
    }

    println!("P2: {}", valid_pos.len());
}
fn main() {
    p_1();
    p_2();
}

