use colored::*;
use std::fs;
use std::collections::HashSet;

type Position = (usize, usize);

fn main() {
    let input_file = "test.txt";
    let mut grid = Vec::<Vec<char>>::new();
    let mut galaxies = Vec::<Position>::new();
    let mut rows_with_galaxy = HashSet::<usize>::new();
    let mut cols_with_galaxy = HashSet::<usize>::new();
    for (i, line) in fs::read_to_string(&input_file)
        .unwrap()
        .lines().enumerate() {
            let mut row = Vec::<char>::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(c),
                    '#' => {
                        row.push(c);
                        galaxies.push((i, j));
                        rows_with_galaxy.insert(i);
                        cols_with_galaxy.insert(j);
                    },
                    _ => {},
                }
            }
            grid.push(row);
        }
    
    let (grid, galaxies) = expand_grid(grid, galaxies, rows_with_galaxy, cols_with_galaxy);

    println!("{galaxies:?}");
    print_grid(&grid);
}

fn expand_grid(mut grid: Vec<Vec<char>>, mut galaxies: Vec<Position>, rows_with_galaxy: HashSet<usize>, cols_with_galaxy: HashSet<usize>) -> (Vec<Vec<char>>, Vec<Position>) {
    let mut rows_to_expand: Vec<usize> = rows_with_galaxy.into_iter().collect();
    rows_to_expand.sort();
    for i in rows_to_expand.into_iter().rev() {
        grid.insert(i, grid[i].clone());
        for g in 0..galaxies.len() {
            if galaxies[g].0 > i {
                galaxies[g] = (galaxies[g].0 + 1, galaxies[g].1)
            }
        }

    }
    (grid, galaxies)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for c in row.iter() {
            match c {
                '.' => print!("{}", c),
                '#' => print!("{}", c.to_string().blue().bold()),
                _ => {},
            }
        }
        print!("\n")
    }
}
