use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input.txt";
    //let filename = "test.txt";
    let _lines = fs::read_to_string(filename).expect("Could not open input file");
    let mut lines = _lines.lines();

    // parse steps
    let steps: Vec<char> = match lines.next() {
        Some(line) => line.chars().collect::<Vec<char>>(),
        None => vec![],
    };
    println!("steps {steps:?}");

    // parse maps
    let mut maps = HashMap::<String, (String, String)>::new();
    let map_pattern = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    for line in lines {
        match map_pattern.captures(line) {
            Some(map_values) => {
                maps.insert(
                    map_values[1].to_string(),
                    (map_values[2].to_string(), map_values[3].to_string()),
                );
            }
            None => {
                println!("no map: {line}");
            }
        }
    }
    println!("{maps:?}");

    // traverse
    let mut step_count: u32 = 0;
    let mut current_val = "AAA";
    while current_val != "ZZZ" {
        for step in &steps {
            let map = maps.get(current_val);
            if let Some(out) = map {
                match step {
                    'L' => {
                        current_val = &out.0;
                        step_count += 1;
                    }
                    'R' => {
                        current_val = &out.1;
                        step_count += 1;
                    }
                    _ => panic!("Unknown Step!"),
                }
            }
        }
    }
    println!("steps: {step_count}");
}
