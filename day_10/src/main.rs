use anyhow::{bail, Result};
use std::fs;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;

fn main() {
    // let input_file = "test.txt";
    let input_file = "input.txt";
    let grid = fs::read_to_string(&input_file)
        .expect("input file not found!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for line in &grid {
        println!("{line:?}");
    }
    let starting_point = find_start(&grid).expect("Couldn't find start");
    println!("starting point: {starting_point:?}");
    let path = find_path(&starting_point, &grid);
    println!("path: {path:?}");
    println!("farthest_point: {:?}", path.len() / 2 - 1);
}

fn find_start(grid: &Grid) -> Result<Position> {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'S' {
                return Ok((i, j));
            }
        }
    }
    bail!("Couldn't find start");
}

fn find_path(start: &Position, grid: &Grid) -> Vec<Position> {
    let mut path = vec![start.clone()];
    let mut current = start.clone();
    let mut next_direction = Direction::Any;
    let mut started = false;
    while !(started && (next_direction == Direction::Any)) {
        println!("looping");
        match next_direction {
            Direction::Any => {
                if current.1 != 0 {
                    if let Some(nd) = check_left(&grid[current.0][current.1 - 1]) {
                        path.push((current.0 - 1, current.1));
                        next_direction = nd;
                    }
                }
                if current.0 != 0 {
                    if let Some(nd) = check_above(&grid[current.0 - 1][current.1]) {
                        path.push((current.0 - 1, current.1));
                        next_direction = nd;
                    }
                }
                if current.0 < grid[0].len() {
                    if let Some(nd) = check_right(&grid[current.0][current.1 + 1]) {
                        path.push((current.0, current.1 + 1));
                        next_direction = nd;
                    }
                }
                if current.1 < grid.len() {
                    if let Some(nd) = check_below(&grid[current.0 + 1][current.1]) {
                        path.push((current.0 + 1, current.1));
                        next_direction = nd;
                    }
                }
                started = true;
            }
            Direction::Up => {
                if let Some(nd) = check_above(&grid[current.0 - 1][current.1]) {
                    path.push((current.0 - 1, current.1));
                    next_direction = nd;
                }
            }
            Direction::Right => {
                if let Some(nd) = check_right(&grid[current.0][current.1 + 1]) {
                    path.push((current.0, current.1 + 1));
                    next_direction = nd;
                }
            }
            Direction::Down => {
                if let Some(nd) = check_below(&grid[current.0 + 1][current.1]) {
                    path.push((current.0 + 1, current.1));
                    next_direction = nd;
                }
            }
            Direction::Left => {
                if let Some(nd) = check_left(&grid[current.0][current.1 - 1]) {
                    path.push((current.0, current.1 - 1));
                    next_direction = nd;
                }
            }
            _ => break,
        }
        current = path[path.len() - 1];
    }

    return path;
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Any,
}

fn check_left(c: &char) -> Option<Direction> {
    match c {
        '-' => Some(Direction::Left),
        'L' => Some(Direction::Up),
        'F' => Some(Direction::Down),
        'S' => Some(Direction::Any),
        _ => None,
    }
}

fn check_above(c: &char) -> Option<Direction> {
    match c {
        '|' => Some(Direction::Up),
        '7' => Some(Direction::Left),
        'F' => Some(Direction::Right),
        'S' => Some(Direction::Any),
        _ => None,
    }
}

fn check_right(c: &char) -> Option<Direction> {
    match c {
        '-' => Some(Direction::Right),
        '7' => Some(Direction::Down),
        'J' => Some(Direction::Up),
        'S' => Some(Direction::Any),
        _ => None,
    }
}

fn check_below(c: &char) -> Option<Direction> {
    match c {
        '|' => Some(Direction::Down),
        'L' => Some(Direction::Right),
        'J' => Some(Direction::Left),
        'S' => Some(Direction::Any),
        _ => None,
    }
}
