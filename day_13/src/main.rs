use std::{cmp, io, iter::zip};

type Grid = Vec<Vec<char>>;

fn main() {
    let mut score_1: usize = 0;
    let mut score_2: usize = 0;
    let mut current_grid: Grid = vec![];
    for l in io::stdin().lines() {
        let line = l.unwrap();
        println!("{}", line);
        if line.len() > 0 {
            current_grid.push(line.chars().collect::<Vec<char>>());
        } else {
            // part 1
            let reflection = find_reflection(&current_grid, 0);
            if let Some(v) = reflection.0 {
                score_1 += v;
            }
            if let Some(v) = reflection.1 {
                score_1 += v * 100;
            }
            // part 2
            let reflection = find_reflection(&current_grid, 1);
            if let Some(v) = reflection.0 {
                score_2 += v;
            }
            if let Some(v) = reflection.1 {
                score_2 += v * 100;
            }
            println!("{reflection:?}");
            current_grid = Vec::<Vec<char>>::new();
        }
    }
    println!("part 1 score: {score_1}");
    println!("part 2 score: {score_2}");
}

fn find_reflection(grid: &Grid, mismatches: usize) -> (Option<usize>, Option<usize>) {
    // check for vertical reflection line
    let width = grid[0].len();
    for column in 1..(width) {
        let compare_lengths = cmp::min(column, width - column);
        if grid
            .iter()
            .map(|row| {
                let left = row[(column - compare_lengths)..column]
                    .iter()
                    .collect::<Vec<&char>>();
                let mut right = row[column..(column + compare_lengths)]
                    .iter()
                    .collect::<Vec<&char>>();
                right.reverse();
                zip(left, right).filter(|(l, r)| l != r).count()
            })
            .sum::<usize>()
            == mismatches
        {
            return (Some(column), None);
        }
    }
    // check for horizontal reflection line
    let height = grid.len();
    for row in 1..height {
        let compare_heights = cmp::min(row, height - row);
        let top = grid[(row - compare_heights)..row]
            .iter()
            .collect::<Vec<&Vec<char>>>();
        let mut bottom = grid[row..(row + compare_heights)]
            .iter()
            .collect::<Vec<&Vec<char>>>();
        bottom.reverse();
        if zip(top, bottom)
            .map(|(t, b)| zip(t, b).filter(|(u, v)| u != v).count())
            .sum::<usize>()
            == mismatches
        {
            return (None, Some(row));
        }
    }
    (None, None)
}
