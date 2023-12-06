use anyhow::Result;
use std::fs;
use std::time::Instant;

fn main() -> Result<()> {
    let mut now = Instant::now();
    println!("part 1: ");
    let filename = "input.txt";
    let _lines = fs::read_to_string(filename).expect("Could not open input file");
    let lines = _lines.lines().collect::<Vec<&str>>();
    let mut times: Vec<u64> = vec![];
    for x in lines[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
    {
        match x.parse::<u64>() {
            Ok(y) => times.push(y),
            Err(e) => println!("{e}"),
        }
    }
    let mut distances: Vec<u64> = vec![];
    for x in lines[1].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
    {
        match x.parse::<u64>() {
            Ok(y) => distances.push(y),
            Err(e) => println!("{e}"),
        }
    }
    let mut mult_result: u64 = 1;
    for i in 0..distances.len() {
        let time = times[i];
        let distance = distances[i];
        let winning_count = calculate_winning_counts(time, distance);
        println!("winnning_count {winning_count}");
        mult_result *= winning_count;
    }
    println!("mult result {mult_result:?}");

    let elapsed_time = now.elapsed();
    println!("Running part 1 took {} ms.", elapsed_time.as_millis());
    now = Instant::now();

    //part 2
    println!("\npart 2: ");
    let mut time_str: Vec<char> = vec![];
    for c in lines[0].split(':').collect::<Vec<&str>>()[1].chars() {
        if c != ' ' {
            time_str.push(c);
        }
    }
    let time = time_str.into_iter().collect::<String>().parse::<u64>()?;

    let mut distance_str: Vec<char> = vec![];
    for c in lines[1].split(':').collect::<Vec<&str>>()[1].chars() {
        if c != ' ' {
            distance_str.push(c);
        }
    }
    let distance = distance_str
        .into_iter()
        .collect::<String>()
        .parse::<u64>()?;
    let winning_count = calculate_winning_counts(time, distance);

    println!("time {time}");
    println!("distance {distance}");
    println!("winning_count {winning_count}");

    let elapsed_time = now.elapsed();
    println!("Running part 2 took {} ms.", elapsed_time.as_millis());

    Ok(())
}

fn calculate_winning_counts(time: u64, distance: u64) -> u64 {
    (0..=time)
        .map(|x| {
            // hold time * acceleration 1m/s/s * time - hold_time)
            if x * 1 * (time - x) > distance {
                1
            } else {
                0
            }
        })
        .sum()
}
