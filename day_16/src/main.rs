use colored::*;
use std::{collections, io};

fn main() {
    let mut grid = Grid {
        raw: vec![],
        map: vec![],
        activated: vec![],
    };
    for l in io::stdin().lines() {
        let line = l.unwrap();
        grid.add_row(&line);

        println!("{}", line);
    }
    let score = grid.find_activated((0, 0, Direction::RIGHT));
    grid.print();
    println!("part 1 activated count: {score}");

    let score = grid.find_best_start();
    println!("part 2 best activated count: {score}");
}

#[derive(Debug, PartialEq)]
enum Space {
    EMPTY,
    SLASH,
    BACKSLASH,
    DASH,
    PIPE,
}

impl Space {
    fn parse(c: &char) -> Space {
        match c {
            '.' => Space::EMPTY,
            '/' => Space::SLASH,
            '\\' => Space::BACKSLASH,
            '-' => Space::DASH,
            '|' => Space::PIPE,
            _ => panic!("Unexpected character '{}'", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Grid {
    raw: Vec<Vec<char>>,
    map: Vec<Vec<Space>>,
    activated: Vec<Vec<bool>>,
}

impl Grid {
    fn add_row(&mut self, line: &str) {
        self.raw.push(line.chars().collect::<Vec<char>>());
        let row = line
            .chars()
            .map(|c| Space::parse(&c))
            .collect::<Vec<Space>>();
        self.activated.push((0..row.len()).map(|_| false).collect());
        self.map.push(row);
    }

    fn find_best_start(&mut self) -> usize {
        let map_height = self.map.len();
        let map_width = self.map[0].len();
        let left_starting_points: Vec<(usize, usize, Direction)> =
            (0..map_height).map(|i| (i, 0, Direction::RIGHT)).collect();
        let mut right_starting_points: Vec<(usize, usize, Direction)> = (0..map_height)
            .map(|i| (i, map_width - 1, Direction::LEFT))
            .collect();
        let mut top_starting_points: Vec<(usize, usize, Direction)> =
            (0..map_width).map(|i| (0, i, Direction::DOWN)).collect();
        let mut bottom_starting_points: Vec<(usize, usize, Direction)> = (0..map_width)
            .map(|i| (map_height - 1, i, Direction::UP))
            .collect();
        let mut starting_points = left_starting_points;
        starting_points.append(&mut right_starting_points);
        starting_points.append(&mut top_starting_points);
        starting_points.append(&mut bottom_starting_points);
        let best = starting_points
            .iter()
            .map(|start| {
                let activated = self.find_activated(start.clone());
                println!("{:?} {}", start, activated);
                activated
            })
            .max();
        match best {
            Some(v) => v,
            None => 0,
        }
    }

    fn find_activated(&mut self, start: (usize, usize, Direction)) -> usize {
        for i in 0..self.activated.len() {
            for j in 0..self.activated[i].len() {
                self.activated[i][j] = false;
            }
        }
        let mut next_positions = collections::VecDeque::<(usize, usize, Direction)>::new();
        next_positions.push_back(start);
        let mut seen = collections::HashSet::<(usize, usize, Direction)>::new();
        loop {
            let position = next_positions.pop_front();
            match position {
                Some(p) => {
                    //println!("{p:?}");
                    let direction = p.2.clone();
                    if (p.0 >= self.map.len()) || (p.1 >= self.map.len()) {
                        continue;
                    }
                    if seen.contains(&(p.0, p.1, p.2.clone())) {
                        continue;
                    } else {
                        seen.insert((p.0, p.1, p.2.clone()));
                    }
                    self.activated[p.0][p.1] = true;
                    let current_space = &self.map[p.0][p.1];
                    match direction {
                        Direction::UP => {
                            match current_space {
                                Space::BACKSLASH => {
                                    if p.1 > 0 {
                                        next_positions.push_back((p.0, p.1 - 1, Direction::LEFT));
                                    }
                                }
                                Space::SLASH => {
                                    next_positions.push_back((p.0, p.1 + 1, Direction::RIGHT));
                                }
                                Space::DASH => {
                                    next_positions.push_back((p.0, p.1 + 1, Direction::RIGHT));
                                    if p.1 > 0 {
                                        next_positions.push_back((p.0, p.1 - 1, Direction::LEFT));
                                    }
                                }
                                _ => {
                                    if p.0 > 0 {
                                        next_positions.push_back((p.0 - 1, p.1, Direction::UP));
                                    }
                                }
                            };
                        }
                        Direction::DOWN => {
                            match current_space {
                                Space::SLASH => {
                                    if p.1 > 0 {
                                        next_positions.push_back((p.0, p.1 - 1, Direction::LEFT));
                                    }
                                }
                                Space::BACKSLASH => {
                                    next_positions.push_back((p.0, p.1 + 1, Direction::RIGHT));
                                }
                                Space::DASH => {
                                    next_positions.push_back((p.0, p.1 + 1, Direction::RIGHT));
                                    if p.1 > 0 {
                                        next_positions.push_back((p.0, p.1 - 1, Direction::LEFT));
                                    }
                                }
                                _ => {
                                    next_positions.push_back((p.0 + 1, p.1, Direction::DOWN));
                                }
                            };
                        }
                        Direction::LEFT => match current_space {
                            Space::BACKSLASH => {
                                if p.0 > 0 {
                                    next_positions.push_back((p.0 - 1, p.1, Direction::UP));
                                }
                            }
                            Space::SLASH => {
                                next_positions.push_back((p.0 + 1, p.1, Direction::DOWN));
                            }
                            Space::PIPE => {
                                next_positions.push_back((p.0 + 1, p.1, Direction::DOWN));
                                if p.0 > 0 {
                                    next_positions.push_back((p.0 - 1, p.1, Direction::UP));
                                }
                            }
                            _ => {
                                if p.1 > 0 {
                                    next_positions.push_back((p.0, p.1 - 1, Direction::LEFT));
                                }
                            }
                        },
                        Direction::RIGHT => match current_space {
                            Space::SLASH => {
                                if p.0 > 0 {
                                    next_positions.push_back((p.0 - 1, p.1, Direction::UP));
                                }
                            }
                            Space::BACKSLASH => {
                                next_positions.push_back((p.0 + 1, p.1, Direction::DOWN));
                            }
                            Space::PIPE => {
                                next_positions.push_back((p.0 + 1, p.1, Direction::DOWN));
                                if p.0 > 0 {
                                    next_positions.push_back((p.0 - 1, p.1, Direction::UP));
                                }
                            }
                            _ => {
                                next_positions.push_back((p.0, p.1 + 1, Direction::RIGHT));
                            }
                        },
                    };
                }
                None => break,
            };
        }
        self.activated
            .iter()
            .map(|row| row.iter().map(|v| *v as usize).sum::<usize>())
            .sum()
    }

    fn print(&self) {
        for (i, row) in self.activated.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c {
                    print!("{}", self.raw[i][j].to_string().red());
                } else {
                    print!("{}", self.raw[i][j].to_string().blue());
                }
            }
            print!("\n");
        }
    }
}
