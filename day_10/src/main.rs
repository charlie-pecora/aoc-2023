use anyhow::{bail, Result};
use std::fs;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;

fn main() {
    // let input_file = "test_2.txt";
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
    // println!("path: {path:?}");
    println!("farthest_point: {:?}", path.len() / 2);

    let enclosed_spaces = find_enclosed_spaces(&grid, &path);
    // println!("enclosed spaces: {:?}", enclosed_spaces);
    println!("number of enclosed spaces: {}", enclosed_spaces.len());
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
        match next_direction {
            Direction::Any => {
                if !started && (current.1 != 0) {
                    if let Some(nd) = check_left(&grid[current.0][current.1 - 1]) {
                        path.push((current.0, current.1 - 1));
                        next_direction = nd;
                        started = true;
                    }
                }
                if !started && (current.0 != 0) {
                    if let Some(nd) = check_above(&grid[current.0 - 1][current.1]) {
                        path.push((current.0 - 1, current.1));
                        next_direction = nd;
                        started = true;
                    }
                }
                if !started && (current.0 < grid[0].len()) {
                    if let Some(nd) = check_right(&grid[current.0][current.1 + 1]) {
                        path.push((current.0, current.1 + 1));
                        next_direction = nd;
                        started = true;
                    }
                }
                if !started && (current.1 < grid.len()) {
                    if let Some(nd) = check_below(&grid[current.0 + 1][current.1]) {
                        path.push((current.0 + 1, current.1));
                        next_direction = nd;
                        started = true;
                    }
                }
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

fn find_enclosed_spaces(grid: &Grid, path: &Vec<Position>) -> Vec<Position> {
    let mut enclosed = Vec::<Position>::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // println!("{i}, {j}");
            let mut passed_above = false;
            let mut passed_below = false;
            let mut passed_left = false;
            let mut passed_right = false;
            let mut is_boundary = false;
            let mut current_quadrant: Option<u8> = None;
            let mut last_quadrant: Option<u8> = None;
            for node in path {
                if (i == node.0) && (j == node.1) {
                    // space is part of boundary
                    is_boundary = true;
                    // println!("is boundary");
                    continue;
                } else if (node.0 < i) && (node.1 < j) {
                    current_quadrant = Some(0);
                } else if (node.0 < i) && (node.1 > j) {
                    current_quadrant = Some(1);
                } else if (node.0 > i) && (node.1 > j) {
                    current_quadrant = Some(2);
                } else if (node.0 > i) && (node.1 < j) {
                    current_quadrant = Some(3);
                }
                if let Some(last) = last_quadrant {
                    if let Some(current) = current_quadrant {
                        if ((last == 0) && (current == 1)) || ((last == 1) && (current == 0)) {
                            passed_left = !passed_left;
                            // println!("passed_left {passed_left}");
                        } else if ((last == 1) && (current == 2)) || ((last == 2) && (current == 1))
                        {
                            passed_below = !passed_below;
                            // println!("passed_below {passed_below}");
                        } else if ((last == 2) && (current == 3)) || ((last == 3) && (current == 2))
                        {
                            passed_right = !passed_right;
                            // println!("passed_right {passed_right}");
                        } else if ((last == 3) && (current == 0)) || ((last == 0) && (current == 3))
                        {
                            passed_above = !passed_above;
                            // println!("passed_above {passed_above}");
                        }
                    }
                }
                last_quadrant = current_quadrant;
            }
            if is_boundary {
                continue;
            } else if passed_above && passed_below && passed_left && passed_right {
                enclosed.push((i, j));
            }
        }
    }
    enclosed
}
