use regex::Regex;
use std::fs;

fn read_input() -> String {
    fs::read_to_string("input").expect("Failed to read the input file")
}

fn p_1() {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = read_input();
    let result: i32 = mul_re
        .captures_iter(&input)
        .map(|cap| {
            cap[1].parse::<i32>().expect("Failed to parse first number") *
                cap[2].parse::<i32>().expect("Failed to parse second number")
        })
        .sum();
    println!("part 1 : {}", result);
}

fn p_2() {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don'?t\(\)|do\(\)").unwrap();
    let input = read_input();
    let mut considered = true;
    let mut result = 0;

    for caps in pattern.captures_iter(&input) {
        if let Some(num1) = caps.get(1) {
            if considered {
                let a: i32 = num1.as_str().parse().expect("Failed to parse first number");
                let b: i32 = caps.get(2).unwrap().as_str().parse().expect("Failed to parse second number");
                result += a * b;
            }
        } else if let Some(m) = caps.get(0) {
            match m.as_str() {
                s if s.starts_with("don't") || s.starts_with("dont") => considered = false,
                "do()" => considered = true,
                _ => {}
            }
        }
    }

    println!("part 2 : {}", result);
}

fn main() {
    p_1();
    p_2();
}
