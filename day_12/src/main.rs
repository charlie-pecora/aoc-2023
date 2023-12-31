use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let mut part_1_possibilities_sum: usize = 0;
    let mut part_2_possibilities_sum: usize = 0;
    let lines = io::stdin().lines();
    for (i, line) in lines.enumerate() {
        println!("line {i}");
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
        // println!("{records:?}");
        // println!("{broken_springs:?}");

        let part_one_possibilities = calculate_possibilities(records.clone(), &broken_springs);
        println!("{part_one_possibilities:?}");
        part_1_possibilities_sum += part_one_possibilities;

        // need to calculate the possibilites with two copies of the parts
        let mut part_2_records = records.clone();
        let mut part_2_broken_springs = broken_springs.clone();
        for _ in 0..1 {
            part_2_records.push('?');
            part_2_records.extend(&records);
            part_2_broken_springs.extend(&broken_springs);
        }
        // println!("{part_2_records:?}");
        // println!("{part_2_broken_springs:?}");
        let two_copy_possibilities = calculate_possibilities(part_2_records, &part_2_broken_springs);
        // use the ratio of possibilities w/ 2 vs 1 copies, then extrapolate to 5
        let ratio = two_copy_possibilities / part_one_possibilities;
        let part_2_possibilities = ratio.pow(4) * part_one_possibilities;
        println!("{part_2_possibilities:?}");
        part_2_possibilities_sum += part_2_possibilities;

    }
    println!("Part 1 total possibilities: {part_1_possibilities_sum}");
    println!("Part 2 total possibilities: {part_2_possibilities_sum}");
    Ok(())
}

fn calculate_possibilities(records: Vec<char>, broken_springs: &Vec<usize>) -> usize {
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
        let is_possible = check_broken_springs(filled_records, broken_springs);
        if is_possible {
            possibilities += 1;
        }
        // println!("{is_possible:?}");
    }

    possibilities
}

fn check_broken_springs(records: Vec<char>, broken_springs: &Vec<usize>) -> bool {
    &get_broken_spring_groups(records) == broken_springs
}

fn get_broken_spring_groups(records: Vec<char>) -> Vec<usize> {
    let mut groups = Vec::<usize>::new();
    let mut current_count: usize = 0;
    for record in records {
        if record == '#' {
            current_count += 1;
        } else if current_count > 0 {
            groups.push(current_count);
            current_count = 0;
        }
    }
    if current_count > 0 {
        groups.push(current_count);
    }
    groups
}
