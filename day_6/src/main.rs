use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let filename = "input.txt";
    let _lines = fs::read_to_string(filename).expect("Could not open input file");
    let lines = _lines.lines().collect::<Vec<&str>>();
    let mut times: Vec<u32> = vec![];
    for x in lines[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
    {
        match x.parse::<u32>() {
            Ok(y) => times.push(y),
            Err(e) => println!("{e}"),
        }
    }
    let mut distances: Vec<u32> = vec![];
    for x in lines[1].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
    {
        match x.parse::<u32>() {
            Ok(y) => distances.push(y),
            Err(e) => println!("{e}"),
        }
    }
    let mut mult_result: u32 = 1;
    for i in 0..distances.len() {
        let time = times[i];
        let distance = distances[i];
        let winning_count: u32 = (0..=time)
            .map(|x| {
                // hold time * acceleration 1m/s/s * time - hold_time)
                if x * 1 * (time - x) > distance {
                    1
                } else {
                    0
                }
            })
            .sum();
        println!("winnning_count {winning_count}");
        mult_result *= winning_count;
    }

    println!("{mult_result:?}");
    Ok(())
}
