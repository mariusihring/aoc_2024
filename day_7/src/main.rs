#[derive(Debug, Clone)]
struct Row {
    goal: usize,
    nums: Vec<usize>,
}

const OPERATORS: [&str; 2] = ["+", "*"];
const OPERATORS_2: [&str; 3] = ["+", "*", "||"];

fn get_rows() -> Vec<Row> {
    let file_content = std::fs::read_to_string("input").unwrap();
    file_content
        .lines()
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let goal: usize = parts[0].parse().unwrap();
            let num_parts = parts[1].split(",").collect::<Vec<&str>>();
            let nums: Vec<usize> = num_parts[0]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            Row { goal, nums }
        })
        .collect::<Vec<Row>>()
}

fn check_combinations(row: &Row) -> bool {
    let num_operators = row.nums.len() - 1;
    let total_combinations = 1 << num_operators;

    for combination in 0..total_combinations {
        let mut result = row.nums[0];

        for pos in 0..num_operators {
            let is_multiply = combination & (1 << pos) != 0;

            let next_num = row.nums[pos + 1];

            if is_multiply {
                result *= next_num;
            } else {
                result += next_num;
            }
        }

        if result == row.goal {
            return true;
        }
    }

    false
}
fn concatenate(a: usize, b: usize) -> usize {
   let b_str = b.to_string();
    a * 10_usize.pow(b_str.len() as u32) + b
}

fn check_combinations_2(row: &Row) -> bool {
    let num_operators = row.nums.len() - 1;
    let total_combinations = 3_usize.pow(num_operators as u32);

    for combination in 0..total_combinations {
        let mut result = row.nums[0];
        let mut temp_combo = combination;

        for pos in 0..num_operators {
            let operator = temp_combo % 3;
            temp_combo /= 3;

            let next_num = row.nums[pos + 1];

            match operator {
                0 => result += next_num,
                1 => result *= next_num,
                2 => result = concatenate(result, next_num),
                _ => unreachable!()
            }
        }

        if result == row.goal {
            return true;
        }
    }

    false
}

fn p_1() {
    let rows = get_rows();
    let mut sum = 0;
    for row in rows {
        if check_combinations(&row) {
            sum += row.goal;
        }
    }

    println!("P1: {}", sum);
}


fn p_2() {
    let rows = get_rows();
    let mut sum = 0;
    for row in rows {
        if check_combinations_2(&row) {
            sum += row.goal;
        }
    }

    println!("P2: {}", sum);
}

fn main() {
    p_1();
    p_2();
}
