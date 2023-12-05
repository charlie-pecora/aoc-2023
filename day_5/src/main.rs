use anyhow::{bail, Result};
use std::fs;

fn main() -> Result<()> {
    let filename = "input.txt";
    let _lines = fs::read_to_string(filename).expect("Could not open input file");
    let mut seeds: Vec<(u64, u64)> = vec![];
    let mut lines = _lines.lines();
    match lines.next() {
        Some(line) => {
            let line_split = line.split(':').collect::<Vec<&str>>();
            let mut last_seed_value: Option<u64> = None;
            for v in line_split[1]
                .trim()
                .split(' ')
                .map(|x| x.parse::<u64>().expect("seeds must produce an int"))
            {
                match last_seed_value {
                    Some(last_val) => {
                        seeds.push((last_val, v));
                        last_seed_value = None;
                    }
                    None => {
                        last_seed_value = Some(v);
                    }
                }
            }
        }
        None => bail!("empty file"),
    };
    let mut current_values = seeds;
    let mut mapped_values: Vec<(u64, u64)> = vec![];
    loop {
        match lines.next() {
            Some(line) => {
                if line.contains("map:") {
                    // process last map and start a new one
                    loop {
                        match current_values.pop() {
                            Some(v) => mapped_values.push(v),
                            None => break,
                        }
                    }
                    current_values = mapped_values;
                    mapped_values = vec![];
                } else {
                    let ranges = line.split(' ').collect::<Vec<&str>>();

                    if let [destination_range_start, source_range_start, range_length] = &ranges[..]
                    {
                        let range_len = range_length.parse::<u64>()?;
                        let dest_range_start = destination_range_start.parse::<u64>()?;
                        let s_range_start = source_range_start.parse::<u64>()?;
                        let mut i: usize = 0;
                        while i < current_values.len() {
                            if (current_values[i] >= s_range_start)
                                && (current_values[i] < s_range_start + range_len)
                            {
                                mapped_values.push(
                                    dest_range_start + current_values.remove(i) - s_range_start,
                                );
                            } else {
                                i += 1;
                            }
                        }
                    }
                }
            }
            None => {
                while current_values.len() > 0 {
                    loop {
                        match current_values.pop() {
                            Some(v) => mapped_values.push(v),
                            None => break,
                        }
                    }
                }
                break;
            }
        };
    }
    println!("{:?}", mapped_values);
    let min_value = mapped_values.iter().min();
    println!("{min_value:?}");
    Ok(())
}
