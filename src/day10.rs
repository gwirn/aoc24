use std::collections::{HashSet, VecDeque};
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn search(grid: &Vec<Vec<i8>>, row: usize, col: usize, start: (i32, i32)) -> usize {
    let mut stack = VecDeque::new();
    let mut found_end: HashSet<(usize, usize)> = HashSet::new();
    stack.push_back(start);
    'outer: loop {
        // println!("{:?}", stack);
        let pos = match stack.pop_front() {
            Some(v) => v,
            None => break 'outer,
        };
        for d in DIRECTIONS {
            // new field to test
            let new_pos = (pos.0 + d.0, pos.1 + d.1);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos_u = (new_pos.0 as usize, new_pos.1 as usize);
            // inside grid
            if new_pos_u.0 < row && new_pos_u.1 < col {
                // valid next step
                if grid[new_pos_u.0][new_pos_u.1] - grid[pos.0 as usize][pos.1 as usize] == 1 {
                    // find top
                    if grid[new_pos_u.0][new_pos_u.1] == 9 {
                        found_end.insert(new_pos_u);
                    }
                    if !stack.contains(&new_pos) {
                        stack.push_back(new_pos);
                    }
                }
            }
        }
    }
    found_end.len()
}

pub fn day10() {
    let raw_input = fs::read_to_string("./inputs/day10.txt").expect("cant read file");

    // generate grid
    let grid = raw_input
        .split("\n")
        .filter(|z| z.len() > 0)
        .into_iter()
        .map(|x| {
            x.split("")
                .filter(|y| y.len() > 0)
                .map(|y| {
                    if y == "." {
                        -1
                    } else {
                        y.parse::<i8>().expect("cant parse to i8")
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let n_row = grid.len();
    let n_col = grid[0].len();
    let mut start_pos = vec![];
    for r in 0..n_row {
        for c in 0..n_col {
            if grid[r][c] == 0 {
                start_pos.push((r as i32, c as i32))
            }
        }
    }
    let mut score = 0;
    for s in &start_pos {
        score += search(&grid, n_row, n_col, *s);
    }
    println!("Score: {}", score);
}
