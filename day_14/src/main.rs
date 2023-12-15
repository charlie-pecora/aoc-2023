use std::{fmt, io};

type Grid = Vec<Vec<Rock>>;

fn main() {
    let mut grid: Grid = vec![];
    for l in io::stdin().lines() {
        let line = l.unwrap();
        for (i, c) in line.chars().enumerate() {
            let rock = Rock::parse(&c);
            if i == grid.len() {
                grid.push(vec![rock]);
            } else {
                grid[i].push(rock);
            }
        }
    }
    print_grid(&grid);
    println!("Part 1 Load: {}", part_1(&grid));
    println!("Part 2 Load: {}", part_2(&grid));
}

#[derive(Debug, Clone)]
enum Rock {
    EMPTY,
    ROUND,
    CUBE,
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self {
            Rock::EMPTY => '.',
            Rock::ROUND => 'O',
            Rock::CUBE => '#',
        };
        write!(f, "{}", v)
    }
}

impl Rock {
    fn parse(c: &char) -> Rock {
        match c {
            'O' => Rock::ROUND,
            '#' => Rock::CUBE,
            _ => Rock::EMPTY,
        }
    }
}

fn part_1(grid: &Grid) -> usize {
    let mut new_grid = grid.to_vec();
    println!("");
    new_grid = shift_up(new_grid);
    print_grid(&new_grid);
    calculate_load(&new_grid)
}

fn part_2(grid: &Grid) -> usize {
    let iters = 1000000000;
    let mut spun_grid = grid.to_vec();
    let mut cycle_len: Option<usize> = None;
    let mut loads: Vec<usize> = vec![];
    for i in 0..iters {
        spun_grid = shift_right(shift_down(shift_left(shift_up(spun_grid))));
        let load = calculate_load(&spun_grid);
        loads.push(load);
        println!("score on step {}: {}", i, load);

        if loads.len() > 1000 {
            for i in 1..(loads.len() / 2) {
                if loads[(loads.len() -2 * i)..(loads.len() - i)] == loads[(loads.len() - i)..] {
                    cycle_len = Some(i);
                    break;
                }
            }
        }
        if let Some(_) = cycle_len {
            break;
        }
    }
    print_grid(&spun_grid);
    if let Some(v) = cycle_len {
        let offset = (iters - loads.len()) % v;
        return loads[loads.len() - v + offset - 1]
    }
    0
}

fn shift_left(mut grid: Grid) -> Grid {
    for row in 0..grid[0].len() {
        let mut support_position: usize = 0;
        let mut rocks_supported: usize = 0;
        for col in 0..grid.len() {
            match grid[col][row] {
                Rock::CUBE => {
                    for r in 0..rocks_supported {
                        grid[support_position + r][row] = Rock::ROUND;
                    }
                    for r in rocks_supported..(col - support_position) {
                        grid[support_position + r][row] = Rock::EMPTY;
                    }
                    support_position = col + 1;
                    rocks_supported = 0;
                }
                Rock::ROUND => rocks_supported += 1,
                _ => {}
            }
        }
        for r in 0..rocks_supported {
            let idx = support_position + r;
            grid[idx][row] = Rock::ROUND;
        }
        for r in rocks_supported..(grid.len() - support_position) {
            let idx = support_position + r;
            grid[idx][row] = Rock::EMPTY;
        }
    }
    grid
}

fn shift_right(mut grid: Grid) -> Grid {
    for row in 0..grid[0].len() {
        let mut rocks_supported: usize = 0;
        for i in 0..grid.len() {
            match grid[i][row] {
                Rock::CUBE => {
                    for r in 0..rocks_supported {
                        grid[i - r - 1][row] = Rock::ROUND;
                    }
                    rocks_supported = 0;
                }
                Rock::ROUND => {
                    grid[i][row] = Rock::EMPTY;
                    rocks_supported += 1;
                }
                _ => {}
            }
        }
        for r in 0..rocks_supported {
            let idx = grid.len() - r - 1;
            grid[idx][row] = Rock::ROUND;
        }
    }
    grid
}

fn shift_down(mut grid: Grid) -> Grid {
    for col in 0..grid.len() {
        let mut rocks_supported: usize = 0;
        for row in 0..grid[col].len() {
            match grid[col][row] {
                Rock::CUBE => {
                    for r in 0..rocks_supported {
                        grid[col][row - r - 1] = Rock::ROUND;
                    }
                    rocks_supported = 0;
                }
                Rock::ROUND => {
                    grid[col][row] = Rock::EMPTY;
                    rocks_supported += 1;
                }
                _ => {}
            }
        }
        for r in 0..rocks_supported {
            let idx = grid[col].len() - r - 1;
            grid[col][idx] = Rock::ROUND;
        }
    }
    grid
}

fn shift_up(mut grid: Grid) -> Grid {
    for col in 0..grid.len() {
        let mut support_position: usize = 0;
        let mut rocks_supported: usize = 0;
        for row in 0..grid[col].len() {
            match grid[col][row] {
                Rock::CUBE => {
                    for r in 0..rocks_supported {
                        grid[col][support_position + r] = Rock::ROUND;
                    }
                    for r in rocks_supported..(row - support_position) {
                        grid[col][support_position + r] = Rock::EMPTY;
                    }
                    support_position = row + 1;
                    rocks_supported = 0;
                }
                Rock::ROUND => rocks_supported += 1,
                _ => {}
            }
        }
        for r in 0..rocks_supported {
            grid[col][support_position + r] = Rock::ROUND;
        }
        for r in rocks_supported..(grid[col].len() - support_position) {
            grid[col][support_position + r] = Rock::EMPTY;
        }
    }
    grid
}

fn calculate_load(grid: &Grid) -> usize {
    let mut load: usize = 0;
    for column in grid {
        let column_height = column.len();
        for (i, rock) in column.iter().enumerate() {
            match rock {
                Rock::ROUND => {
                    load += column_height - i;
                }
                _ => {}
            }
        }
    }
    load
}

fn print_grid(grid: &Grid) {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            print!("{}", grid[row][col]);
        }
        print!("\n");
    }
}
