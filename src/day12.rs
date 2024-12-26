use std::{collections::VecDeque, fs};
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIAGONALS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

/*
calculate the number of faces by calculating the number of edges
*/
fn edge_test(plant_fields: &Vec<(usize, usize)>) -> usize {
    // only type conversion
    let pf_i32: Vec<(i32, i32)> = plant_fields
        .iter()
        .map(|x| (x.0 as i32, x.1 as i32))
        .collect();
    let mut egdes = 0;
    // for each plant field check if if L fields are the same and the diagonal is not or if the
    // L fields are different - both shows an edge
    for p in plant_fields {
        for i in 0..4 {
            // first L field
            let d0 = (p.0 as i32 + DIRECTIONS[i].0, p.1 as i32 + DIRECTIONS[i].1);
            // diagonal field
            let mut d1 = (0, 0);
            if i == 3 {
                d1.0 = p.0 as i32 + DIRECTIONS[0].0;
                d1.1 = p.1 as i32 + DIRECTIONS[0].1;
            } else {
                d1.0 = p.0 as i32 + DIRECTIONS[i + 1].0;
                d1.1 = p.1 as i32 + DIRECTIONS[i + 1].1;
            }
            // second L field
            let di0 = (p.0 as i32 + DIAGONALS[i].0, p.1 as i32 + DIAGONALS[i].1);
            // if the needed fields exist
            let d0_test = pf_i32.contains(&d0);
            let d1_test = pf_i32.contains(&d1);
            let di0_test = pf_i32.contains(&di0);
            if d0_test && d1_test && !di0_test || !d0_test && !d1_test {
                egdes += 1;
            }
        }
    }
    egdes
}
/*
flood fill after finding new starting point and calculate cost
*/
fn flood(grid: &mut Vec<Vec<&str>>, yd: usize, xd: usize) -> (usize, usize) {
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
        let field = match queue.pop_front() {
            Some(v) => {
                visited.push(v);
                area += 1;
                v
            }
            None => break,
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
    let edges = edge_test(&visited);
    // calculate cost
    (area * perimeter, area * edges)
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
    let mut cost1 = 0;
    let mut cost2 = 0;
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
        let (p_cost, e_cost) = flood(&mut grid, ydim, xdim);
        cost1 += p_cost;
        cost2 += e_cost;
    }
    println!("Total cost part 1: {}", cost1);
    println!("Total cost part 2: {}", cost2);
}
