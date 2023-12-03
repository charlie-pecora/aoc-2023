use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let filename = "input.txt";
    // let filename = "test.txt";
    let mut part_number_sum: u32 = 0;
    let mut total_gear_ratio: u32 = 0;
    let mut last_line_chars: Vec<SpecialChar> = vec![];
    let mut last_line_numbers: Vec<Number> = vec![];
    for line in fs::read_to_string(filename)
        .expect("Couldn't read input file")
        .lines()
    {
        let mut part_number_line_sum: u32 = 0;
        let mut line_chars: Vec<SpecialChar> = vec![];
        let mut line_numbers: Vec<Number> = vec![];
        let mut current_num_str: Vec<char> = vec![];
        for (i, c) in line.chars().enumerate() {
            if (!c.is_numeric()) && (c != '.') {
                line_chars.push(SpecialChar { i, v: c , gears: vec![]});
            }
            if c.is_numeric() {
                current_num_str.push(c)
            } else if current_num_str.len() != 0 {
                line_numbers.push(Number {
                    start_index: i - current_num_str.len(),
                    end_index: i - 1,
                    v: current_num_str.iter().collect::<String>().parse::<u32>()?,
                    used: false,
                });
                current_num_str = vec![];
            }
        }
        if current_num_str.len() != 0 {
            let i = line.len();
            line_numbers.push(Number {
                start_index: i - current_num_str.len(),
                end_index: i - 1,
                v: current_num_str.iter().collect::<String>().parse::<u32>()?,
                used: false,
            });
        }
        for number in &mut last_line_numbers.iter_mut() {
            for c in &mut line_chars.iter_mut() {
                if (c.i + 1 >= number.start_index)
                    && (c.i <= number.end_index + 1)
                {
                    if c.v == '*' {
                        c.gears.push(number.v);
                    }
                    if !number.used {
                        part_number_line_sum += number.v;
                        number.used = true;
                    }
                }
            }
        }
        for number in &mut line_numbers.iter_mut() {
            for c in &mut line_chars.iter_mut() {
                if (c.i + 1 >= number.start_index)
                    && (c.i <= number.end_index + 1)
                    && (!number.used)
                {
                    if c.v == '*' {
                        c.gears.push(number.v);
                    }
                    if !number.used {
                        part_number_line_sum += number.v;
                        number.used = true;
                    }
                }
            }
            for c in &mut last_line_chars.iter_mut() {
                if (c.i + 1 >= number.start_index)
                    && (c.i <= number.end_index + 1)
                    && (!number.used)
                {
                    if c.v == '*' {
                        c.gears.push(number.v);
                    }
                    if !number.used {
                        part_number_line_sum += number.v;
                        number.used = true;
                    }
                }
            }
        }
        for c in &last_line_chars {
            if c.gears.len() == 2 {
                total_gear_ratio += c.gears[0] * c.gears[1];
            }
        }
        part_number_sum += part_number_line_sum;
        println!("{:?}, {:?}", last_line_chars, last_line_numbers);
        // println!("{:?}, {:?}", line_chars, line_numbers);
        println!("{}", line);
        println!("{}", part_number_line_sum);
        last_line_chars = line_chars;
        last_line_numbers = line_numbers;
    }
    println!("{:?}, {:?}", last_line_chars, last_line_numbers);
    println!("part # sum: {}", part_number_sum);
    println!("Total Gear Ratio: {}", total_gear_ratio);
    Ok(())
}

#[derive(Debug)]
struct SpecialChar {
    i: usize,
    v: char,
    gears: Vec<u32>,
}

#[derive(Debug)]
struct Number {
    start_index: usize,
    end_index: usize,
    v: u32,
    used: bool,
}
