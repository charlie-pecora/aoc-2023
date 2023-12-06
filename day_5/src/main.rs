use anyhow::{bail, Result};
use std::fs;

fn main() -> Result<()> {
    let filename = "input.txt";
    //let filename = "test.txt";
    let _lines = fs::read_to_string(filename).expect("Could not open input file");
    let mut seeds: Vec<Range> = vec![];
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
                        seeds.push(Range {
                            start: last_val,
                            length: v,
                        });
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
    let mut mapped_values: Vec<Range> = vec![];
    loop {
        match lines.next() {
            Some(line) => {
                if line.contains("map:") {
                    // process last map and start a new one
                    println!("{line}");
                    loop {
                        match current_values.pop() {
                            Some(v) => mapped_values.push(v),
                            None => break,
                        }
                    }
                    current_values = mapped_values;
                    mapped_values = vec![];
                } else {
                    //println!("{current_values:?} {mapped_values:?}");
                    let ranges = line.split(' ').collect::<Vec<&str>>();

                    if let [destination_range_start, source_range_start, range_length] = &ranges[..]
                    {
                        let range_len = range_length.parse::<u64>()?;
                        let dest_range_start = destination_range_start.parse::<u64>()?;
                        let s_range_start = source_range_start.parse::<u64>()?;
                        let mut i: usize = 0;
                        while i < current_values.len() {
                            let val = current_values[i];
                            if (val.start + val.length <= s_range_start)
                                || (val.start >= s_range_start + range_len)
                            {
                                // no overlap
                                i += 1;
                            } else if (val.start >= s_range_start)
                                && (val.start < s_range_start + range_len)
                            {
                                // start is within the range
                                if val.start + val.length <= s_range_start + range_len {
                                    // source range is fully contained
                                    let mut moved = current_values.remove(i);
                                    moved.start = dest_range_start + moved.start - s_range_start;
                                    mapped_values.push(moved);
                                } else {
                                    // source range starts in range, but overflows
                                    let new_range = Range {
                                        start: dest_range_start + current_values[i].start
                                            - s_range_start,
                                        length: s_range_start + range_len - val.start,
                                    };
                                    mapped_values.push(new_range);
                                    current_values[i].start = val.start + range_len;
                                    current_values[i].length =
                                        val.start + val.length - range_len - s_range_start;
                                    i += 1;
                                }
                            } else if (val.start + val.length > s_range_start)
                                && (val.start + val.length <= s_range_start + range_len)
                            {
                                // end is within the source range
                                let new_range = Range {
                                    start: dest_range_start,
                                    length: val.start + val.length - s_range_start,
                                };
                                mapped_values.push(new_range);
                                current_values[i].length = s_range_start - val.start;
                                i += 1;
                            } else {
                                // source range fully contains the map
                                let moved = current_values.remove(i);
                                let new_range = Range {
                                    start: dest_range_start,
                                    length: range_len,
                                };
                                mapped_values.push(new_range);
                                // handle remainders
                                current_values.push(Range {
                                    start: moved.start,
                                    length: s_range_start - moved.start,
                                });
                                current_values.push(Range {
                                    start: s_range_start + range_len,
                                    length: moved.start + moved.length - s_range_start - range_len,
                                });
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
    // println!("{:?}", mapped_values);
    let min_value = mapped_values.iter().map(|x| x.start.clone()).min();
    println!("{min_value:?}");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    length: u64,
}
