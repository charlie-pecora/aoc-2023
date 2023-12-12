use colored::*;
use std::{cmp, collections::HashSet, fs};

type Position = (usize, usize);

fn main() {
    // let input_file = "test.txt";
    let input_file = "input.txt";
    let mut grid = Vec::<Vec<char>>::new();
    let mut galaxies = Vec::<Position>::new();
    let mut rows_with_galaxy = HashSet::<usize>::new();
    let mut cols_with_galaxy = HashSet::<usize>::new();
    for (i, line) in fs::read_to_string(&input_file).unwrap().lines().enumerate() {
        let mut row = Vec::<char>::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(c),
                '#' => {
                    row.push(c);
                    galaxies.push((i, j));
                    rows_with_galaxy.insert(i);
                    cols_with_galaxy.insert(j);
                }
                _ => {}
            }
        }
        grid.push(row);
    }
    print_grid(&grid);

    let part_1_galaxies = expand_grid(&grid, &galaxies, &rows_with_galaxy, &cols_with_galaxy, 2);
    let part_1_total_distance = calculate_total_distance(&part_1_galaxies);
    println!("Part 1: total distance sum: {part_1_total_distance}");

    let part_2_galaxies = expand_grid(
        &grid,
        &galaxies,
        &rows_with_galaxy,
        &cols_with_galaxy,
        1_000_000,
    );
    let part_2_total_distance = calculate_total_distance(&part_2_galaxies);
    println!("Part 2: total distance sum: {part_2_total_distance}");
}

fn expand_grid(
    grid: &Vec<Vec<char>>,
    galaxies: &Vec<Position>,
    rows_with_galaxy: &HashSet<usize>,
    cols_with_galaxy: &HashSet<usize>,
    expansion_factor: usize,
) -> Vec<Position> {
    let mut expanded_galaxies = galaxies.clone();
    for i in (0..grid.len()).rev() {
        if !rows_with_galaxy.contains(&i) {
            for g in 0..expanded_galaxies.len() {
                if expanded_galaxies[g].0 > i {
                    expanded_galaxies[g] = (
                        expanded_galaxies[g].0 + expansion_factor - 1,
                        expanded_galaxies[g].1,
                    )
                }
            }
        }
    }
    for j in (0..grid[0].len()).rev() {
        if !cols_with_galaxy.contains(&j) {
            for g in 0..expanded_galaxies.len() {
                if expanded_galaxies[g].1 > j {
                    expanded_galaxies[g] = (
                        expanded_galaxies[g].0,
                        expanded_galaxies[g].1 + expansion_factor - 1,
                    );
                }
            }
        }
    }
    expanded_galaxies
}

fn calculate_total_distance(galaxies: &Vec<Position>) -> usize {
    let mut total_distances: usize = 0;
    for (i, g_0) in galaxies.iter().enumerate() {
        for g_1 in galaxies[i + 1..].iter() {
            let distance = cmp::max(g_0.0, g_1.0) - cmp::min(g_0.0, g_1.0) + cmp::max(g_0.1, g_1.1)
                - cmp::min(g_0.1, g_1.1);
            total_distances += distance;
        }
    }
    total_distances
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for c in row.iter() {
            match c {
                '.' => print!("{}", c),
                '#' => print!("{}", c.to_string().blue().bold()),
                _ => {}
            }
        }
        print!("\n")
    }
}
