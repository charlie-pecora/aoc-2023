use std::fs;

const digit_names: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let filename = "input.txt";
    let mut calibration_codes = Vec::<u32>::new();
    let mut calibration_codes_sum: u32 = 0;

    for line in fs::read_to_string(filename).unwrap().lines() {
        let digit = parse_digits_2(line);
        match digit {
            Some(v) => {
                calibration_codes.push(v);
                calibration_codes_sum += v;
            }
            None => {}
        };
    }
    println!("{:?}", calibration_codes);
    println!("{:?}", calibration_codes_sum);
}

fn parse_digits(line: &str) -> Option<u32> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    if digits.len() != 0 {
        let digit =
            digits[0].to_digit(10).unwrap() * 10 + digits[digits.len() - 1].to_digit(10).unwrap();
        return Some(digit);
    } else {
        return None;
    }
}

fn parse_digits_2(line: &str) -> Option<u32> {
    let mut digits: Vec<u32> = vec![];
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            digits.push(c.to_digit(10).unwrap());
        } else {
            for (digit_name, digit_value) in digit_names {
                let mut max_index = i + digit_name.len();
                if max_index > line.len() {
                    max_index = line.len();
                }
                if &&line[i..max_index] == digit_name {
                    digits.push(digit_value.clone());
                    break;
                }
            }
        }
    }
    if digits.len() != 0 {
        let digit = digits[0] * 10 + digits[digits.len() - 1];
        return Some(digit);
    } else {
        return None;
    }
}
