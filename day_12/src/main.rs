use itertools::Itertools;
use std::io;

fn main() -> io::Result<()> {
    let mut part_1_possibilities_sum: usize = 0;
    let mut part_2_possibilities_sum: usize = 0;
    let lines = io::stdin().lines();
    for line in lines {
        let l = line?;
        let parts = l.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        let broken_springs = parts[1]
            .split(',')
            .map(|x| x.parse::<usize>().expect("can't parse into int"))
            .collect::<Vec<usize>>();
        let records = parts[0].chars().collect::<Vec<char>>();

        let possibilities = calculate_possibilities(&records, &broken_springs);
        part_1_possibilities_sum += possibilities;

        let mut part_2_records = records.clone();
        let mut part_2_broken_springs = broken_springs.clone();
        for _ in 0..5 {
            part_2_records.push('?');
            part_2_records.extend(&records);
            part_2_broken_springs.extend(&broken_springs);
        }
        println!("{part_2_records:?}");
        println!("{part_2_broken_springs:?}");
        let possibilities = calculate_possibilities(&part_2_records, &part_2_broken_springs);
        println!("{possibilities:?}");
        part_2_possibilities_sum += possibilities;
    }
    println!("Part 1 total possibilities: {part_1_possibilities_sum}");
    println!("Part 2 total possibilities: {part_2_possibilities_sum}");
    Ok(())
}

fn calculate_possibilities(records: &Vec<char>, broken_springs: &Vec<usize>) -> usize {
    let needed: usize =
        broken_springs.iter().sum::<usize>() - records.iter().filter(|x| x == &&'#').count();
    let mut unknown_indices = Vec::<usize>::new();
    for (i, record) in records.iter().enumerate() {
        if record == &'?' {
            unknown_indices.push(i);
        }
    }
    let mut possibilities: usize = 0;
    for selection in unknown_indices.iter().combinations(needed) {
        let mut filled_records = records.clone();
        for s in selection {
            filled_records[*s] = '#';
        }
        // println!("{filled_records:?}");
        let is_possible = check_broken_springs(&filled_records, broken_springs);
        if is_possible {
            possibilities += 1;
        }
        // println!("{is_possible:?}");
    }

    possibilities
}

fn check_broken_springs(records: &Vec<char>, broken_springs: &Vec<usize>) -> bool {
    let mut broken_springs_index: usize = 0;
    let mut current_count: usize = 0;
    for record in records {
        if record == &'#' {
            current_count += 1;
        } else if current_count > 0 {
            if broken_springs[broken_springs_index] == current_count {
                current_count = 0;
                broken_springs_index += 1;
            } else {
                return false;
            }
        }
        if broken_springs_index == broken_springs.len() {
            return true;
        }
    }
    if current_count > 0 {
        if broken_springs[broken_springs_index] == current_count {
            if broken_springs_index + 1 == broken_springs.len() {
                return true;
            }
        }
    }
    false
}
