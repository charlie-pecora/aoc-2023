use anyhow::{bail, Result};
use std::{cmp::min, collections::HashMap, fs};

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
    println!("path: {path:?}");
    println!("farthest_point: {:?}", path.len() / 2);

    let updated_grid = set_starting_char(grid, &path);

    let enclosed_spaces = find_enclosed_spaces(&updated_grid, &path);
    println!("enclosed spaces: {:?}", enclosed_spaces);
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

fn set_starting_char(mut grid: Grid, path: &Vec<Position>) -> Grid {
    let first_position = path[0];
    let second_position = path[1];
    let last_position = path[path.len() - 2];
    let mut starting_char: Option<char> = None;
    if (second_position.0 != first_position.0) && (last_position.0 != first_position.0) {
        starting_char = Some('|');
    } else if (second_position.1 != first_position.1) && (last_position.1 != first_position.1) {
        starting_char = Some('-');
    } else if second_position.0 < first_position.0 {
        if last_position.1 > first_position.1 {
            starting_char = Some('L');
        } else if last_position.1 < first_position.1 {
            starting_char = Some('J');
        }
    } else if second_position.0 > first_position.0 {
        if last_position.1 > first_position.1 {
            starting_char = Some('F');
        } else if last_position.1 < first_position.1 {
            starting_char = Some('7');
        }
    } else if last_position.0 < first_position.0 {
        if second_position.1 > first_position.1 {
            starting_char = Some('L');
        } else if second_position.1 < first_position.1 {
            starting_char = Some('J');
        }
    } else if last_position.0 > first_position.0 {
        if second_position.1 > first_position.1 {
            starting_char = Some('F');
        } else if second_position.1 < first_position.1 {
            starting_char = Some('7');
        }
    }
    if let Some(c) = starting_char {
        grid[first_position.0][first_position.1] = c;
    }
    grid
}

fn find_enclosed_spaces(grid: &Grid, path: &Vec<Position>) -> Vec<Position> {
    let mut enclosed = Vec::<Position>::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let mut is_boundary = false;
            let mut above = HashMap::<char, u32>::new();
            let mut below = HashMap::<char, u32>::new();
            let mut left = HashMap::<char, u32>::new();
            let mut right = HashMap::<char, u32>::new();
            for node in path {
                if i == node.0 {
                    if j == node.1 {
                        // space is part of boundary
                        is_boundary = true;
                    } else if node.1 > j {
                        let count = right.entry(grid[node.0][node.1]).or_insert(0);
                        *count += 1;
                    } else {
                        let count = left.entry(grid[node.0][node.1]).or_insert(0);
                        *count += 1;
                    }
                } else if j == node.1 {
                    if node.0 > i {
                        let count = above.entry(grid[node.0][node.1]).or_insert(0);
                        *count += 1;
                    } else {
                        let count = below.entry(grid[node.0][node.1]).or_insert(0);
                        *count += 1;
                    }
                }
            }
            if is_boundary {
                continue;
            }

            if closed_vertically(&left) {
                if closed_vertically(&right) {
                    if closed_horizontally(&above) {
                        if closed_horizontally(&below) {
                            enclosed.push((i, j));
                        }
                    }
                }
            }
        }
    }
    enclosed
}

fn closed_vertically(entries: &HashMap<char, u32>) -> bool {
    let mut score = 0;
    if let Some(v) = entries.get(&'|') {
        score += v;
    }
    let mut compat_chars_1 = 0;
    let mut compat_chars_2 = 0;
    if let Some(v) = entries.get(&'F') {
        compat_chars_1 += v;
    }
    if let Some(v) = entries.get(&'7') {
        compat_chars_1 += v;
    }
    if let Some(v) = entries.get(&'J') {
        compat_chars_2 += v;
    }
    if let Some(v) = entries.get(&'L') {
        compat_chars_2 += v;
    }
    score += min(compat_chars_1, compat_chars_2);
    score % 2 == 1
}

fn closed_horizontally(entries: &HashMap<char, u32>) -> bool {
    let mut score = 0;
    if let Some(v) = entries.get(&'-') {
        score += v;
    }
    let mut compat_chars_1 = 0;
    let mut compat_chars_2 = 0;
    if let Some(v) = entries.get(&'F') {
        compat_chars_1 += v;
    }
    if let Some(v) = entries.get(&'7') {
        compat_chars_2 += v;
    }
    if let Some(v) = entries.get(&'J') {
        compat_chars_2 += v;
    }
    if let Some(v) = entries.get(&'L') {
        compat_chars_1 += v;
    }
    score += min(compat_chars_1, compat_chars_2);
    score % 2 == 1
}
