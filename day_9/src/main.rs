use std::fs;

fn main() {
    let input_file = "input.txt";
    // let input_file = "test.txt";
    let mut predicted_values: Vec<i64> = vec![];
    let mut historical_values: Vec<i64> = vec![];
    for line in fs::read_to_string(&input_file)
        .expect("File {input_file} doesn't exist")
        .lines()
    {
        let readings = parse_line(line);
        println!("{readings:?}");
        match predict_last_and_next(readings) {
            Some(v) => {
                historical_values.push(v.0);
                predicted_values.push(v.1);
            }
            None => {}
        }
    }
    println!("predicted values: {predicted_values:?}");
    let predicted_values_sum: i64 = predicted_values.iter().sum();
    println!("predicted values sum: {predicted_values_sum:?}");

    println!("historical values: {historical_values:?}");
    let historical_values_sum: i64 = historical_values.iter().sum();
    println!("historical values sum: {historical_values_sum:?}");
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn predict_last_and_next(readings: Vec<i64>) -> Option<(i64, i64)> {
    if readings.len() == 0 {
        return None;
    }
    let mut next_val: i64 = 0;
    let mut diffs: Vec<Vec<i64>> = vec![readings];
    while !diffs[diffs.len() - 1].iter().all(|x| x == &0_i64) {
        let last_diff = &diffs[diffs.len() - 1];
        next_val += last_diff[last_diff.len() - 1];
        let mut new_diff: Vec<i64> = vec![];
        for i in 0..(last_diff.len() - 1) {
            new_diff.push(last_diff[i + 1] - last_diff[i]);
        }
        diffs.push(new_diff);
    }
    let mut last_val: i64 = 0;
    for i in (0..diffs.len()).rev() {
        last_val = diffs[i][0] - last_val;
    }
    println!("{diffs:?}");
    Some((last_val, next_val))
}
