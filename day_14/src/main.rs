use std::{io, fmt};

fn main() {
    let mut grid: Vec<Vec<Rock>> = vec![];
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
}

#[derive(Debug)]
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

fn part_1(grid: &Vec<Vec<Rock>>) -> usize {
    let mut load: usize = 0;
    for column in grid {
        let column_height = column.len();
        let mut support_position: usize = 0;
        let mut rocks_supported: usize = 0;
        for (i, rock) in column.iter().enumerate() {
            match rock {
                Rock::CUBE => {
                    for r in 0..rocks_supported {
                        load += column_height - support_position - (r);
                    }
                    support_position = i + 1;
                    rocks_supported = 0;
                },
                Rock::ROUND => rocks_supported += 1,
                _ => {},
            }
        }
        for r in 0..rocks_supported {
            load += column_height - support_position - (r);
        }
    }
    load
}

fn print_grid(grid: &Vec<Vec<Rock>>) {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            print!("{}", grid[row][col]);
        }
        print!("\n");
    }
}
