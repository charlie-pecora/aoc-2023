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
    let map_pattern = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
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

    // part 1 traverse
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
    println!("part 1 steps: {step_count}");

    // part 2 traverse
    let current_vals = maps
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    let mut counts: Vec<u64> = vec![];
    let mut cumulative_lcm: u64 = 1;
    for val in current_vals {
        let mut current_val = val;
        let mut step_count: u64 = 0;
        while !current_val.ends_with("Z") {
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
        cumulative_lcm = lcm(cumulative_lcm, step_count);
        println!("count: {step_count}");
        counts.push(step_count);
    }
    println!("part 2 answer: {cumulative_lcm} individual counts: {counts:?}");
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
