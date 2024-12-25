use std::{collections::VecDeque, fs};
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

/*
flood fill after finding new starting point
*/
fn flood(grid: &mut Vec<Vec<&str>>, yd: usize, xd: usize) -> usize {
    let mut area = 0;
    let mut perimeter = 0;
    // find next new field to start (first not visited field)
    let mut start = (0, 0);
    'outer: for y in 0..yd {
        for x in 0..xd {
            if grid[y][x] != "." {
                start.0 = y;
                start.1 = x;
                break 'outer;
            }
        }
    }
    // the current plant type
    let current_plant = grid[start.0][start.1];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);
    // the visited fields of the current plant
    let mut visited: Vec<(usize, usize)> = Vec::new();
    loop {
        if queue.is_empty() {
            break;
        }
        let field = match queue.pop_front() {
            Some(v) => {
                visited.push(v);
                area += 1;
                v
            }
            None => panic!("pop from empty queue"),
        };
        let mut peri = 4;
        // check for all directions if there is the same plant
        for d in DIRECTIONS {
            let dy = field.0 as i32 + d.0;
            let dx = field.1 as i32 + d.1;
            if dy >= 0 && dy < yd as i32 && dx >= 0 && dx < xd as i32 {
                let dyu = dy as usize;
                let dxu = dx as usize;
                let valid_next = grid[dyu][dxu] == current_plant;
                // only add if not visited or already queued
                if !visited.contains(&(dyu, dxu)) && !queue.contains(&(dyu, dxu)) && valid_next {
                    queue.push_back((dyu, dxu));
                }
                // reduce perimeter if it has a neighbor
                if valid_next {
                    peri -= 1;
                }
            }
        }
        perimeter += peri;
    }
    // mask visited plant fields in the grid
    for i in &visited {
        grid[i.0][i.1] = ".";
    }
    // calculate cost
    area * perimeter
}
pub fn day12() {
    let raw_input = fs::read_to_string("./inputs/day12.txt").expect("cant open file");
    // generate grid
    let mut grid: Vec<Vec<&str>> = raw_input
        .split("\n")
        .filter(|y| y.len() > 0)
        .map(|x| x.split("").filter(|z| z.len() > 0).collect())
        .collect();
    // grid dimensions
    let ydim = grid.len();
    let xdim = grid[0].len();
    // total fence cost
    let mut cost = 0;
    loop {
        // if all grid spaces are mask it's finished
        if &grid
            .iter()
            .map(|x| if x.iter().all(|y| *y == ".") { 1 } else { 0 })
            .sum::<usize>()
            == &ydim
        {
            break;
        }
        cost += flood(&mut grid, ydim, xdim);
    }
    println!("Total cost: {}", cost);
}
