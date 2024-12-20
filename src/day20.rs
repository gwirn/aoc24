use std::{collections::HashMap, fs};
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

/*
calculate the Manhattan distance between two 2D points
*/
fn manhattan_dist(x: (usize, usize), y: (usize, usize)) -> i32 {
    (y.0 as i32 - x.0 as i32).abs() + (y.1 as i32 - x.1 as i32).abs()
}

pub fn day20() {
    // PART 1
    let raw_data = fs::read_to_string("./inputs/day20.txt").expect("cant read file");
    let lines: Vec<&str> = raw_data.split("\n").filter(|x| x.len() > 0).collect();
    // generate grid from input
    let mut grid: Vec<Vec<&str>> = lines
        .iter()
        .map(|x| {
            x.split("")
                .into_iter()
                .filter(|y| y.len() > 0)
                .collect::<Vec<&str>>()
        })
        .collect();
    // grid dimensions
    let n_y = grid.len();
    let n_x = grid[0].len();

    // coordinates that are paths
    let mut path_coords: Vec<(usize, usize)> = vec![];
    // coordinates that are walls
    let mut wall_holes: Vec<((usize, usize), (usize, usize))> = vec![];

    // find start and end and replace them in the grid to path points
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (ci, i) in grid.clone().iter().enumerate() {
        match i.iter().position(|x| *x == "S") {
            Some(idx) => {
                start.0 = ci;
                start.1 = idx;
                grid[ci][idx] = "."
            }
            None => {}
        }
        match i.iter().position(|x| *x == "E") {
            Some(idx) => {
                end.0 = ci;
                end.1 = idx;
                grid[ci][idx] = "."
            }
            None => {}
        }
    }

    for y in 1..n_y - 1 {
        for x in 1..n_x - 1 {
            let cur_field = grid[y][x];
            // path coordinates
            if cur_field == "." {
                path_coords.push((y, x));
            // wall coordinates
            } else if cur_field == "#" {
                // opposing sites
                let right_dir_pair = (
                    (y as i32 + DIRECTIONS[0].0) as usize,
                    (x as i32 + DIRECTIONS[0].1) as usize,
                );
                let left_dir_pair = (
                    (y as i32 + DIRECTIONS[1].0) as usize,
                    (x as i32 + DIRECTIONS[1].1) as usize,
                );
                let down_dir_pair = (
                    (y as i32 + DIRECTIONS[2].0) as usize,
                    (x as i32 + DIRECTIONS[2].1) as usize,
                );
                let up_dir_pair = (
                    (y as i32 + DIRECTIONS[3].0) as usize,
                    (x as i32 + DIRECTIONS[3].1) as usize,
                );
                // if a wall has path field on is opposing sites it can be used as shortcut
                if grid[up_dir_pair.0][up_dir_pair.1] == "."
                    && grid[down_dir_pair.0][down_dir_pair.1] == "."
                {
                    wall_holes.push((up_dir_pair, down_dir_pair))
                }
                if grid[left_dir_pair.0][left_dir_pair.1] == "."
                    && grid[right_dir_pair.0][right_dir_pair.1] == "."
                {
                    wall_holes.push((left_dir_pair, right_dir_pair))
                }
            }
        }
    }
    // index of the start coordinates in the path coordinates
    let path_start_idx = path_coords
        .iter()
        .position(|x| *x == start)
        .expect("cant find position");
    path_coords.swap(0, path_start_idx);
    let n_pc = path_coords.len();
    // reconstruct path in order
    for p in 0..n_pc - 1 {
        'inner: for d in DIRECTIONS {
            match &path_coords[p + 1..].iter().position(|x| {
                *x == (
                    (path_coords[p].0 as i32 + d.0) as usize,
                    (path_coords[p].1 as i32 + d.1) as usize,
                )
            }) {
                Some(idx) => {
                    path_coords.swap(p + 1, p + 1 + idx);
                    break 'inner;
                }
                None => {
                    continue;
                }
            };
        }
    }
    let mut gains: HashMap<i32, usize> = HashMap::new();
    // calculate the gains made how often by getting the positions of the shortcut path coordinates
    // in the path and calculating how many path points lie between them minus the 2 ps the
    // shortcut takes
    for i in &wall_holes {
        let hole0 = path_coords
            .iter()
            .position(|x| *x == i.0)
            .expect("cant find position") as i32;
        let hole1 = path_coords
            .iter()
            .position(|x| *x == i.1)
            .expect("cant find position") as i32;
        let cheat_gain = (hole1 - hole0).abs() - 2;
        *gains.entry(cheat_gain).or_default() += 1;
    }
    // get the fast enough shortcuts
    let mut n_fastest = 0;
    for i in gains {
        if i.0 >= 100 {
            n_fastest += i.1;
        }
    }
    println!("{:?}", n_fastest);

    // PART 2
    let mut gains20: HashMap<i32, usize> = HashMap::new();
    let limit: i32 = 20;
    // for each point in the path calculate the distance it takes to all other remaining path
    // points and if they are closer than 21 ps calculate the path steps in between them
    // method like for PART 1
    for p in 0..n_pc - 1 {
        let pdists = &path_coords[p + 1..n_pc]
            .iter()
            .map(|x| {
                let xdist = manhattan_dist(path_coords[p], *x);
                xdist
            })
            .collect::<Vec<i32>>();
        for (cp, pd) in pdists.iter().enumerate() {
            // calculate gain
            if pd <= &limit {
                let pd_test = &path_coords[p + 1..n_pc][cp];
                let p_test = &path_coords[p];
                let hole0 = path_coords
                    .iter()
                    .position(|x| *x == *p_test)
                    .expect("cant find position") as i32;
                let hole1 = path_coords
                    .iter()
                    .position(|x| *x == *pd_test)
                    .expect("cant find position") as i32;
                let cheat_gain = (hole1 - hole0).abs() - pd;
                *gains20.entry(cheat_gain).or_default() += 1;
            }
        }
    }
    let mut n_fastest = 0;
    for i in gains20 {
        if i.0 >= 100 {
            n_fastest += i.1;
        }
    }
    println!("{:?}", n_fastest);
}
