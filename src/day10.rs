use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn search(grid: &Vec<Vec<i8>>, row: usize, col: usize, start: (i32, i32)) -> (usize, usize) {
    let mut stack = VecDeque::new();
    // coordinates of the found trail end
    let mut found_end: HashSet<(usize, usize)> = HashSet::new();
    // ad start to stack
    stack.push_back(start);
    // how often a specific coordinate was visited (would only need the data for the trail end
    // coordinates but these are not known before
    // how often the top is visited is also the number of different ways to the top
    let mut n_visited: HashMap<(i32, i32), usize> = HashMap::new();
    // max number of hiking trails from trail head
    let mut n_ways_max: HashMap<(i32, i32), usize> = HashMap::new();
    'outer: loop {
        // number of times a coordinate appears in the stack
        let mut stack_count: HashMap<(i32, i32), usize> = HashMap::new();
        for i in stack.iter() {
            *stack_count.entry(*i).or_insert(0) += 1;
        }
        // check how often a coordinate was visited
        for i in stack_count.iter() {
            match n_visited.get(i.0) {
                Some(v) => {
                    if i.1 > v {
                        n_visited.insert(*i.0, *i.1);
                    }
                }
                None => {
                    n_visited.insert(*i.0, *i.1);
                }
            }
        }
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
                        // update number of times visited
                        match n_visited.get(&new_pos) {
                            Some(v) => {
                                let nwm = match n_ways_max.get(&new_pos) {
                                    Some(v) => v,
                                    None => &0,
                                };
                                if *v > *nwm {
                                    n_ways_max.insert(new_pos, *v);
                                }
                            }
                            None => {}
                        }
                    }
                    stack.push_back(new_pos);
                }
            }
        }
    }
    (
        // PART 1
        found_end.len(),
        // PART 2
        n_ways_max.iter().map(|(_, x)| x).sum::<usize>() + found_end.len(),
    )
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
    let mut score1 = 0;
    let mut score2 = 0;
    for s in &start_pos {
        let (s1, s2) = search(&grid, n_row, n_col, *s);
        score1 += s1;
        score2 += s2;
    }
    println!("Score Part 1: {}", score1);
    println!("Score Part 2: {}", score2);
}
